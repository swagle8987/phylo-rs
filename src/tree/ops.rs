use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use num::{Float, Num, NumCast, Signed};
use std::{fmt::{Debug, Display}, hash::Hash};

use crate::{iter::node_iter::Ancestors, node::simple_rnode::{RootedMetaNode, RootedTreeNode}, tree::simple_rtree::RootedMetaTree};
use super::{Clusters, EulerWalk, RootedTree, DFS};

pub trait SPR: RootedTree + DFS + Sized
{
    /// Attaches input tree to self by spliting an edge
    fn graft(&mut self, tree: Self, edge: (Self::NodeID, Self::NodeID));

    /// Returns subtree starting at given node, while corresponding nodes from self.
    fn prune(&mut self, node_id: Self::NodeID)-> Self;

    /// SPR function
    fn spr(&mut self, edge1: (Self::NodeID, Self::NodeID), edge2: (Self::NodeID, Self::NodeID))
    {
        let pruned_tree = SPR::prune(self, edge1.1);
        SPR::graft(self, pruned_tree, edge2);
    }
}

pub trait NNI
where
    Self: RootedTree + Sized,
{
    fn nni(&mut self, parent_id: Self::NodeID);
}

pub trait Reroot
where
    Self: RootedTree + Sized,
{
    fn reroot_at_node(&mut self, node_id: Self::NodeID);
    fn reroot_at_edge(&mut self, edge: (Self::NodeID, Self::NodeID));
}

pub trait Balance: Clusters + SPR + Sized
where
    Self::NodeID: Display + Debug + Hash + Clone + Ord,
{
    fn balance_subtree(&mut self);
}

pub trait Subtree: Ancestors + DFS + Sized
where
    Self::NodeID: Display + Debug + Hash + Clone + Ord,
{
    fn induce_tree(&self, node_id_list: impl IntoIterator<Item=Self::NodeID, IntoIter = impl ExactSizeIterator<Item = Self::NodeID>>)->Self
    {
        let new_root_id = self.get_root_id();
        let mut subtree: Self = RootedTree::new(new_root_id);
        for node_id in node_id_list.into_iter()
        {
            let ancestors = self.root_to_node(node_id);
            subtree.set_nodes(ancestors);
        }
        subtree.clean();
        subtree
    }
    fn subtree(&self, node_id: Self::NodeID)->Self
    {
        let new_root_id = self.get_root_id();
        let mut subtree: Self = RootedTree::new(new_root_id);
        let dfs = self.dfs(node_id);
        subtree.set_nodes(dfs);
        subtree

    }
}

pub trait RobinsonFoulds
where
    Self: RootedTree + Sized,
    Self::NodeID: Display + Debug + Hash + Clone + Ord,
{
    fn rfs(&self, tree: Self)->usize;
}

pub trait ClusterAffinity
where
    Self: RootedTree + Sized,
    Self::NodeID: Display + Debug + Hash + Clone + Ord,
{
    fn ca(&self, tree: Self)->usize;
}

pub trait WeightedRobinsonFoulds
{
    fn wrfs(&self, tree: Self)->usize;
}

pub trait CopheneticDistance<T>: RootedMetaTree<Meta=<Self as CopheneticDistance<T>>::Meta> + EulerWalk + Clusters + Ancestors + Sized
where
    T: Signed + Clone + NumCast + std::iter::Sum + Debug + Display + Float + PartialOrd + Copy,
    <Self as RootedTree>::Node: RootedMetaNode<Meta=<Self as CopheneticDistance<T>>::Meta>
{
    type Meta: Display + Debug + Eq + PartialEq + Clone + Ord + Hash;

    fn cophen_dist(&self, tree: &Self, zeta_func: fn(&Self, Self::NodeID) -> T, norm: usize)->T
    {
        let double_mix_type = self.distance_double_mix_type(tree, zeta_func, norm);
        let non_mix_type = self.distance_non_mix_type(tree, zeta_func);
        let single_mix_type = self.distance_single_mix_type(tree, zeta_func);
        let distance =  dbg!(double_mix_type)+dbg!(non_mix_type)+single_mix_type;

        return distance.powf(T::from(norm).unwrap().powi(-1))

        // self.cophen_dist_naive(tree, zeta_func, norm)
    }

    fn cophen_dist_naive(&self, tree: &Self, zeta_func: fn(&Self, Self::NodeID) -> T, norm: usize)->T
    {
        let t_taxa_map = self.get_leaves().into_iter().map(|x| (x.get_taxa().unwrap(), x)).collect::<HashMap<<Self as CopheneticDistance<T>>::Meta, Self::Node>>();
        let t_hat_taxa_map = tree.get_leaves().into_iter().map(|x| (x.get_taxa().unwrap(), x)).collect::<HashMap<<Self as CopheneticDistance<T>>::Meta, Self::Node>>();

        let t_euler_walk = self.euler_walk(self.get_root_id()).into_iter().map(|x| x.get_id()).collect_vec();
        let t_hat_euler_walk = tree.euler_walk(tree.get_root_id()).into_iter().map(|x| x.get_id()).collect_vec();

        let binding1 = t_taxa_map.keys().map(|x| x.clone()).collect::<HashSet<<Self as CopheneticDistance<T>>::Meta>>();
        let binding2 = t_hat_taxa_map.keys().map(|x| x.clone()).collect::<HashSet<<Self as CopheneticDistance<T>>::Meta>>();
        let taxa = binding1.intersection(&binding2).collect_vec();

        Self::compute_norm(taxa.iter().combinations(2)
        .map(|a| (a[0], a[1]))
        .map(|(m1, m2)| {
            let t_lca_id = self.get_lca_id(t_taxa_map.get(&m1).cloned().unwrap().get_id(), t_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_euler_walk.clone()));
            let t_hat_lca_id = tree.get_lca_id(t_hat_taxa_map.get(&m1).cloned().unwrap().get_id(), t_hat_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_hat_euler_walk.clone()));
                return (zeta_func(&self, t_lca_id.clone())-zeta_func(&tree, t_hat_lca_id.clone())).abs()
            }), norm)
    }

    fn get_cntr(&self, leaf_set: HashSet<Self::NodeID>, zeta_func: fn(&Self, Self::NodeID) -> T)->Vec<T>
    {
        // line 5 in algo 1
        let mut gamma: Vec<T> = Vec::new();
        let median_node_id = self.get_median_node_id();
        let euler_walk = self.euler_walk(self.get_root_id()).into_iter().map(|x| x.get_id()).collect_vec();
        // line 3 in algo 1
        let mut median_path = self.node_to_root(median_node_id.clone()).into_iter().map(|x| (x.get_id(), 0)).collect::<HashMap<Self::NodeID, u32>>();
        for node_id in leaf_set{
            // line 4 in algo 1
            median_path.entry(self.get_lca_id(node_id, median_node_id.clone(), Some(euler_walk.clone()))).and_modify(|x| *x+=1);
        }
        for (node_id, c) in median_path{
            for _ in 0..c{
                gamma.push(zeta_func(&self, node_id.clone()))
            }
        }
        gamma
    }

    fn compute_norm(vector: impl Iterator<Item=T>, norm: usize)->T
    {
        vector.map(|x| x.powi(norm as i32)).sum::<T>().powf(T::from(norm).unwrap().powi(-1))
    }

    fn seq_product(alpha: Vec<T>, beta:Vec<T>, norm: usize)->T
    {
        let (mut alpha, mut beta) = match alpha.clone().into_iter().reduce(|a, b| a.max(b))>beta.clone().into_iter().reduce(|a, b| a.max(b)){
            true => { (beta.clone(), alpha.clone())},
            false => { (alpha.clone(), beta.clone()) },
        };
        alpha.sort_by(|a, b| a.partial_cmp(b).unwrap());
        beta.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if alpha.len()==0 || beta.len()==0 {return T::zero();}
        if alpha.len()==1 && beta.len()==1 {
            return beta[0].clone()-alpha[0].clone()
        }
        let (mut i,mut j) = (0,0);
        let mut sigma: Vec<Vec<T>> = vec![vec![T::zero(); norm]; beta.len()];
        while j< beta.len()-1{
            if i<alpha.len()-1 && alpha[i]<=beta[j]{
                for l in 0..norm{
                    sigma[i][l] = sigma[i][l] + alpha[i].powi(l as i32);
                }
                i += 1;
            }
            else{
                j +=1;
                for l in 0..norm{
                    sigma[j][l] = sigma[j-1][l];
                }
            }
        }
        T::zero()
        // return T::from(beta.len()).unwrap_or(T::one())*(alpha.iter().map(|x| x.clone()).sum()) + (0..beta.len()).map(|j|  T::from(2).unwrap_or(T::one())*T::from(lamda[j]).unwrap_or(T::one())*beta[j].clone()-T::from(2).unwrap_or(T::one())*sigma[j].clone()-T::from(alpha.iter().len()).unwrap_or(T::one())*beta[j].clone()).sum();
    }

    /// This method generates the distance contributed (raised to the p^{th} power) by all taxa pairs 
    /// that are present in the same subtree on both trees.
    /// 
    /// This includes the following assignments: AA|A'A', AA|B'B', BB|B'B', BB|A'A'.
    fn distance_non_mix_type(&self, tree: &Self, zeta_func: fn(&Self, Self::NodeID)->T)->T
    {
        let t_taxa_map = self.get_leaves().into_iter().map(|x| (x.get_taxa().unwrap(), x)).collect::<HashMap<<Self as CopheneticDistance<T>>::Meta, Self::Node>>();
        let t_hat_taxa_map = tree.get_leaves().into_iter().map(|x| (x.get_taxa().unwrap(), x)).collect::<HashMap<<Self as CopheneticDistance<T>>::Meta, Self::Node>>();

        let t_euler_walk = self.euler_walk(self.get_root_id()).into_iter().map(|x| x.get_id()).collect_vec();
        let t_hat_euler_walk = tree.euler_walk(tree.get_root_id()).into_iter().map(|x| x.get_id()).collect_vec();

        let t:Self::NodeID = self.get_median_node_id();
        let t_hat = tree.get_median_node_id();
        
        let b: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(self.get_cluster(t).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap()));
        let b_hat: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(tree.get_cluster(t_hat).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap()));

        let a: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(self.get_cluster(self.get_root_id()).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap())).difference(&b).map(|x| x.clone()).collect();
        let a_hat: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(tree.get_cluster(tree.get_root_id()).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap())).difference(&b_hat).map(|x| x.clone()).collect();

        let a_int_a_hat: HashSet<<Self as RootedMetaTree>::Meta> = a.intersection(&a_hat).map(|x| x.clone()).collect();
        let a_int_b_hat: HashSet<<Self as RootedMetaTree>::Meta> = a.intersection(&b_hat).map(|x| x.clone()).collect();
        let b_int_a_hat: HashSet<<Self as RootedMetaTree>::Meta> = b.intersection(&a_hat).map(|x| x.clone()).collect();
        let b_int_b_hat: HashSet<<Self as RootedMetaTree>::Meta> = b.intersection(&b_hat).map(|x| x.clone()).collect();

        // d_{AA|A'A'}
        let d1: T = a_int_a_hat.iter().combinations(2)
                                .map(|a| (a[0].clone(), a[1].clone()))
                                .map(|(m1, m2)| {
                                    let t_lca_id = self.get_lca_id(t_taxa_map.get(&m1).cloned().unwrap().get_id(), t_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_euler_walk.clone()));
                                    let t_hat_lca_id = tree.get_lca_id(t_hat_taxa_map.get(&m1).cloned().unwrap().get_id(), t_hat_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_hat_euler_walk.clone()));
                                    return (zeta_func(&self, t_lca_id.clone())-zeta_func(&tree, t_hat_lca_id.clone())).abs()
                                })
                                .sum();
        let d2: T = a_int_b_hat.iter().combinations(2)
                            .map(|a| (a[0].clone(), a[1].clone()))
                            .map(|(m1, m2)| {
                                let t_lca_id = self.get_lca_id(t_taxa_map.get(&m1).cloned().unwrap().get_id(), t_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_euler_walk.clone()));
                                let t_hat_lca_id = tree.get_lca_id(t_hat_taxa_map.get(&m1).cloned().unwrap().get_id(), t_hat_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_hat_euler_walk.clone()));
                                if zeta_func(&self, t_lca_id.clone())>zeta_func(&tree, t_hat_lca_id.clone()){
                                    return zeta_func(&self, t_lca_id.clone())-zeta_func(&tree, t_hat_lca_id.clone())
                                }
                                else{
                                    return zeta_func(&tree, t_hat_lca_id)-zeta_func(&self, t_lca_id)
                                }
                            })
                            .sum();
        let d3: T = b_int_a_hat.iter().combinations(2)
                        .map(|a| (a[0].clone(), a[1].clone()))
                        .map(|(m1, m2)| {
                            let t_lca_id = self.get_lca_id(t_taxa_map.get(&m1).cloned().unwrap().get_id(), t_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_euler_walk.clone()));
                            let t_hat_lca_id = tree.get_lca_id(t_hat_taxa_map.get(&m1).cloned().unwrap().get_id(), t_hat_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_hat_euler_walk.clone()));
                            if zeta_func(&self, t_lca_id.clone())>zeta_func(&tree, t_hat_lca_id.clone()){
                                return zeta_func(&self, t_lca_id.clone())-zeta_func(&tree, t_hat_lca_id.clone())
                            }
                            else{
                                return zeta_func(&tree, t_hat_lca_id)-zeta_func(&self, t_lca_id)
                            }
                        })
                        .sum();
        let d4: T = b_int_b_hat.iter().combinations(2)
                    .map(|a| (a[0].clone(), a[1].clone()))
                    .map(|(m1, m2)| {
                        let t_lca_id = self.get_lca_id(t_taxa_map.get(&m1).cloned().unwrap().get_id(), t_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_euler_walk.clone()));
                        let t_hat_lca_id = tree.get_lca_id(t_hat_taxa_map.get(&m1).cloned().unwrap().get_id(), t_hat_taxa_map.get(&m2).cloned().unwrap().get_id(), Some(t_hat_euler_walk.clone()));
                        if zeta_func(&self, t_lca_id.clone())>zeta_func(&tree, t_hat_lca_id.clone()){
                            return zeta_func(&self, t_lca_id.clone())-zeta_func(&tree, t_hat_lca_id.clone())
                        }
                        else{
                            return zeta_func(&tree, t_hat_lca_id)-zeta_func(&self, t_lca_id)
                        }
                    })
                    .sum();
                
        return d1+d2+d3+d4;
    }

    /// This method generates the distance contributed by all taxa pairs 
    /// that are present in the same subtree in exactly one of the two trees(raised to the p^{th} power).
    /// 
    /// This includes the following assignments: AA|A'B', AA|B'A', BB|A'B', BB|B'A', BA|B'B', BA|A'A', AB|B'B', AB|A'A'.
    fn distance_single_mix_type(&self, tree: &Self, zeta_func: fn(&Self, Self::NodeID)->T)->T
    {
        let vertex_beta:HashMap<Self::NodeID, T> = HashMap::new();
        let vertex_delta:HashMap<Self::NodeID, (usize, T)> = HashMap::new();
        let vertex_sig_p:HashMap<Self::NodeID, (usize, T)> = HashMap::new();
        let vertex_sig_m:HashMap<Self::NodeID, (usize, T)> = HashMap::new();

        let t_taxa_map = self.get_leaves().into_iter().map(|x| (x.get_taxa().unwrap(), x)).collect::<HashMap<<Self as CopheneticDistance<T>>::Meta, Self::Node>>();
        let t_hat_taxa_map = tree.get_leaves().into_iter().map(|x| (x.get_taxa().unwrap(), x)).collect::<HashMap<<Self as CopheneticDistance<T>>::Meta, Self::Node>>();

        let t:Self::NodeID = self.get_median_node_id();
        let t_hat = tree.get_median_node_id();
        
        let b: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(self.get_cluster(t).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap()));
        let b_hat: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(tree.get_cluster(t_hat).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap()));

        let a: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(self.get_cluster(self.get_root_id()).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap())).difference(&b).map(|x| x.clone()).collect();
        let a_hat: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(tree.get_cluster(tree.get_root_id()).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap())).difference(&b_hat).map(|x| x.clone()).collect();

        // A \cap A'
        let a_int_a_hat: HashSet<<Self as RootedMetaTree>::Meta> = a.intersection(&a_hat).map(|x| x.clone()).collect();


        
        T::zero()
    }

    /// This method generates the distance contributed by all taxa pairs 
    /// that are present in different subtrees in both trees(raised to the p^{th} power).
    /// 
    /// This includes the following assignments: AB|A'B', AB|B'A', BA|B'A', BA|A'B'
    fn distance_double_mix_type(&self, tree: &Self, zeta_func: fn(&Self, Self::NodeID)->T, norm: usize)->T
    {
        let t_taxa_map = self.get_leaves().into_iter().map(|x| (x.get_taxa().unwrap(), x)).collect::<HashMap<<Self as CopheneticDistance<T>>::Meta, Self::Node>>();
        let t_hat_taxa_map = tree.get_leaves().into_iter().map(|x| (x.get_taxa().unwrap(), x)).collect::<HashMap<<Self as CopheneticDistance<T>>::Meta, Self::Node>>();

        let t:Self::NodeID = self.get_median_node_id();
        let t_hat = tree.get_median_node_id();
        
        let b: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(self.get_cluster(t).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap()));
        let b_hat: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(tree.get_cluster(t_hat).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap()));

        let a: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(self.get_cluster(self.get_root_id()).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap())).difference(&b).map(|x| x.clone()).collect();
        let a_hat: HashSet<<Self as RootedMetaTree>::Meta> = HashSet::from_iter(tree.get_cluster(tree.get_root_id()).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap())).difference(&b_hat).map(|x| x.clone()).collect();

        // AB|B'A'
        let a_int_b_hat: HashSet<<Self as RootedMetaTree>::Meta> = a.intersection(&b_hat).map(|x| x.clone()).collect();
        let b_int_a_hat: HashSet<<Self as RootedMetaTree>::Meta> = b.intersection(&a_hat).map(|x| x.clone()).collect();

        // AB|A'B'
        let a_int_a_hat: HashSet<<Self as RootedMetaTree>::Meta> = a.intersection(&a_hat).map(|x| x.clone()).collect();
        let b_int_b_hat: HashSet<<Self as RootedMetaTree>::Meta> = b.intersection(&b_hat).map(|x| x.clone()).collect();

        let alpha_1 = self.get_cntr(a_int_b_hat.iter().map(|x| t_taxa_map.get(x).cloned().unwrap().get_id()).collect::<HashSet<Self::NodeID>>(), zeta_func);
        let beta_1 = tree.get_cntr(b_int_a_hat.iter().map(|x| t_hat_taxa_map.get(x).cloned().unwrap().get_id()).collect::<HashSet<Self::NodeID>>(), zeta_func);

        let alpha_2 = self.get_cntr(a_int_a_hat.iter().map(|x| t_taxa_map.get(x).cloned().unwrap().get_id()).collect::<HashSet<Self::NodeID>>(), zeta_func);
        let beta_2 = tree.get_cntr(b_int_b_hat.iter().map(|x| t_hat_taxa_map.get(x).cloned().unwrap().get_id()).collect::<HashSet<Self::NodeID>>(), zeta_func);

        return Self::seq_product(alpha_1, beta_1, norm) + Self::seq_product(alpha_2, beta_2, norm);
    }
}
