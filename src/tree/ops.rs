use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use num::{Num, NumCast};
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

pub trait CopheneticDistance<T>: RootedMetaTree<Taxa=<Self as CopheneticDistance<T>>::Taxa> + EulerWalk + Clusters + Ancestors + Sized
where
    T: Num + Clone + Ord + NumCast + std::iter::Sum,
    <Self as RootedTree>::Node: RootedMetaNode<Taxa=<Self as CopheneticDistance<T>>::Taxa>
{
    type Taxa: Display + Debug + Eq + PartialEq + Clone + Ord + Hash;

    fn cophen_dist(&self, tree: Self, zeta_func: fn(&Self, Self::Node) -> T)->T
    {
        let t:Self::NodeID = self.get_median_node_id();
        let t_hat = tree.get_median_node_id();
        
        let b: HashSet<<Self as RootedMetaTree>::Taxa> = HashSet::from_iter(self.get_cluster(t).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap()));
        let b_hat: HashSet<<Self as RootedMetaTree>::Taxa> = HashSet::from_iter(tree.get_cluster(t_hat).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap()));

        let a: HashSet<<Self as RootedMetaTree>::Taxa> = HashSet::from_iter(self.get_cluster(self.get_root_id()).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap())).difference(&b).map(|x| x.clone()).collect();
        let a_hat: HashSet<<Self as RootedMetaTree>::Taxa> = HashSet::from_iter(tree.get_cluster(tree.get_root_id()).into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap())).difference(&b_hat).map(|x| x.clone()).collect();

        let a_int_b_hat: HashSet<<Self as RootedMetaTree>::Taxa> = dbg!(a.intersection(&b_hat).map(|x| x.clone()).collect());
        let b_int_a_hat: HashSet<<Self as RootedMetaTree>::Taxa> = dbg!(b.intersection(&a_hat).map(|x| x.clone()).collect());

        let alpha = self.get_cntr(a_int_b_hat, zeta_func);
        let beta = tree.get_cntr(b_int_a_hat, zeta_func);

        return Self::seq_product(alpha, beta);
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
            median_path.entry(self.get_lca(node_id, median_node_id.clone(), Some(euler_walk.clone()))).and_modify(|x| *x+=1);
        }
        for (node_id, c) in median_path{
            for _ in 0..c{
                gamma.push(zeta_func(&self, node_id.clone()))
            }
        }
        gamma
    }

    fn seq_product(alpha: Vec<T>, beta:Vec<T>)->T
    {
        let (mut alpha, mut beta) = match alpha.iter().max()>beta.iter().max(){
            true => { (beta.clone(), alpha.clone())},
            false => { (alpha.clone(), beta.clone()) },
        };
        alpha.sort();
        beta.sort();
        let (mut i,mut j) = (0,0);
        let mut sigma: Vec<T> = vec![T::zero(); beta.len()];
        let mut lamda: Vec<u32> = vec![0; beta.len()];
        while j< beta.len(){
            if i<alpha.len() && alpha[i]<=beta[j]{
                sigma[i] += alpha[i].clone();
                lamda[j] += 1;
                i += 1;
            }
            else{
                j +=1;
                sigma[j] = sigma[j-1].clone();
                lamda[j] = lamda[j-1];
            }
        }
        return T::from(beta.len()).unwrap_or(T::one())*(alpha.iter().map(|x| x.clone()).sum()) + (0..beta.len()).map(|j|  T::from(2).unwrap_or(T::one())*T::from(lamda[j]).unwrap_or(T::one())*beta[j].clone()-T::from(2).unwrap_or(T::one())*sigma[j].clone()-T::from(alpha.iter().len()).unwrap_or(T::one())*beta[j].clone()).sum();
    }

    fn distance_double_mix_type(&self, tree: &Self)->T
    {
        let t = self.get_median_node_id();
        let t_hat = tree.get_median_node_id();

        let A = self.get_cluster(t.clone()).into_iter().map(|x| x.get_id()).collect_vec();
        T::zero()
    }
}
