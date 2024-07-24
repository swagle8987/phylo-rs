use std::{fmt::{Display, Debug}, hash::Hash};
use num::{Float, NumCast, Signed};
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use anyhow::Result;

use crate::node::simple_rnode::{RootedZetaNode, RootedWeightedNode};
use crate::tree::{RootedTree, TreeNodeID, TreeNodeMeta, RootedWeightedTree, TreeNodeWeight};
use crate::tree::{Clusters, Ancestors, RootedMetaTree, ContractTree, RootedMetaNode};


/// A type alias for zeta annotation of a node in a tree.
pub type TreeNodeZeta<'a, T> = <<T as RootedTree<'a>>::Node as RootedZetaNode>::Zeta;

/// A trait describing the path functions in a tree.
pub trait PathFunction<'a>: RootedTree<'a>
where
    <Self as RootedTree<'a>>::Node: RootedZetaNode,
{
    /// Sets zeta value of all nodes in a tree by function
    fn set_zeta(&'a mut self, zeta_func: fn(&Self, TreeNodeID<'a, Self>) -> TreeNodeZeta<'a, Self>);
    
    /// Returns zeta value of a node in a tree. None is no zeta value is set
    fn get_zeta(&self, node_id: TreeNodeID<'a, Self>) -> Result<TreeNodeZeta<'a, Self>>;

    /// Returns true if node zeta value is not None
    fn is_zeta_set(&self, node_id: TreeNodeID<'a, Self>) -> bool;

    /// Returns true if all node zeta value is not None    
    fn is_all_zeta_set(&self) -> bool;

    /// Sets zeta value of a node in a tree by value.
    fn set_node_zeta(
        &mut self,
        node_id: TreeNodeID<'a, Self>,
        zeta: Option<TreeNodeZeta<'a, Self>>,
    );
}

/// A trait describing naive computation of Robinson Foulds distance
pub trait RobinsonFoulds<'a>
where
    Self: RootedTree<'a> + Sized,
    TreeNodeID<'a, Self>: Display + Debug + Hash + Clone + Ord,
{
    /// Returns Robinson Foulds distance between tree and self.
    fn rfs(&self, tree: &Self) -> usize;
}

/// A trait describing naive computation of Cluster Affinity distance
pub trait ClusterAffinity<'a>
where
    Self: RootedTree<'a> + Sized,
{
    /// Returns Cluster Affinity distance between tree and self.
    fn ca(&self, tree: &Self) -> usize;
}

/// A trait describing naive computation of Weighted Robinson Foulds distance
pub trait WeightedRobinsonFoulds<'a> 
where 
    Self: RootedWeightedTree<'a> + Sized,
    <Self as RootedTree<'a>>::Node: RootedWeightedNode,
{
    /// Returns weighted Robinson Foulds distance between tree and self.
    fn wrfs(&self, tree: &Self) -> TreeNodeWeight<'a, Self>;
}

/// A trait describing naive computation of cophenetic distance
pub trait CopheneticDistance<'a>:
    PathFunction<'a>
    + RootedMetaTree<'a>
    + Clusters<'a>
    + Ancestors<'a>
    + ContractTree<'a>
    + Debug
    + Sized
where
    <Self as RootedTree<'a>>::Node: RootedMetaNode<'a> + RootedZetaNode,
    <<Self as RootedTree<'a>>::Node as RootedZetaNode>::Zeta: Signed
        + Clone
        + NumCast
        + std::iter::Sum
        + Debug
        + Display
        + Float
        + PartialOrd
        + Copy
        + Send,
{
    /// Returns zeta of leaf by taxa
    fn get_zeta_taxa(
        &'a self,
        taxa: &TreeNodeMeta<'a, Self>,
    ) -> <<Self as RootedTree>::Node as RootedZetaNode>::Zeta {
        self.get_zeta(self.get_taxa_node_id(taxa).unwrap()).unwrap()
    }

    /// Reurns the nth norm of an iterator composed of floating point values
    fn compute_norm(
        vector: impl Iterator<Item = <<Self as RootedTree<'a>>::Node as RootedZetaNode>::Zeta>,
        norm: u32,
    ) -> <<Self as RootedTree<'a>>::Node as RootedZetaNode>::Zeta {
        if norm == 1 {
            return vector.sum();
        }
        vector
            .map(|x| x.powi(norm as i32))
            .sum::<<<Self as RootedTree>::Node as RootedZetaNode>::Zeta>()
            .powf(
                <<<Self as RootedTree>::Node as RootedZetaNode>::Zeta as NumCast>::from(norm)
                    .unwrap()
                    .powi(-1),
            )
    }

    /// Reurns the cophenetic distance between two trees using the naive algorithm (\Theta(n^2))
    fn cophen_dist_naive(
        &'a self,
        tree: &'a Self,
        norm: u32,
    ) -> <<Self as RootedTree>::Node as RootedZetaNode>::Zeta {
        if !self.is_all_zeta_set() || !tree.is_all_zeta_set() {
            panic!("Zeta values not set");
        }
        let binding1 = self
            .get_taxa_space()
            .collect::<HashSet<&<<Self as RootedTree<'a>>::Node as RootedMetaNode>::Meta>>();
        let binding2 = tree
            .get_taxa_space()
            .collect::<HashSet<&<<Self as RootedTree<'a>>::Node as RootedMetaNode>::Meta>>();
        let taxa_set = binding1.intersection(&binding2).cloned();

        self.cophen_dist_naive_by_taxa(tree, norm, taxa_set)
    }

    /// Returns the Cophenetic distance between two trees restricted to a taxa set using the \theta(n^2) naive algorithm.
    fn cophen_dist_naive_by_taxa(
        &'a self,
        tree: &'a Self,
        norm: u32,
        taxa_set: impl Iterator<Item = &'a TreeNodeMeta<'a, Self>> + Clone,
    ) -> <<Self as RootedTree>::Node as RootedZetaNode>::Zeta {
        let taxa_set = taxa_set.collect_vec();
        let cophen_vec = taxa_set
            .iter()
            .combinations_with_replacement(2)
            .map(|x| match x[0] == x[1] {
                true => vec![x[0]],
                false => x,
            })
            .map(|x| match x.len() {
                1 => {
                    let zeta_1 = self.get_zeta_taxa(x[0]);
                    let zeta_2 = tree.get_zeta_taxa(x[0]);
                    (zeta_1 - zeta_2).abs()
                }
                _ => {
                    let self_ids = x
                        .iter()
                        .map(|a| self.get_taxa_node_id(a).unwrap())
                        .collect_vec();
                    let tree_ids = x
                        .iter()
                        .map(|a| tree.get_taxa_node_id(a).unwrap())
                        .collect_vec();
                    let t_lca_id = self.get_lca_id(self_ids.as_slice());
                    let t_hat_lca_id = tree.get_lca_id(tree_ids.as_slice());
                    let zeta_1 = self.get_zeta(t_lca_id).unwrap();
                    let zeta_2 = tree.get_zeta(t_hat_lca_id).unwrap();
                    (zeta_1 - zeta_2).abs()
                }
            })
            .collect_vec();

        Self::compute_norm(cophen_vec.into_iter(), norm)
    }
}
