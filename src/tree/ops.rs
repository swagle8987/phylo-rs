use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;

use itertools::Itertools;
use num::{Float, NumCast, Signed};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use super::{distances::PathFunction, Clusters, EulerWalk, RootedTree, RootedZetaNode, DFS};
use crate::{
    iter::node_iter::Ancestors,
    node::simple_rnode::{RootedMetaNode, RootedTreeNode},
    tree::simple_rtree::{RootedMetaTree, TreeNodeID, TreeNodeMeta},
};

pub trait SPR<'a>: RootedTree<'a> + DFS<'a> + Sized {
    /// Attaches input tree to self by spliting an edge
    fn graft(&mut self, tree: Self, edge: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>));

    /// Returns subtree starting at given node, while corresponding nodes from self.
    fn prune(&mut self, node_id: TreeNodeID<'a, Self>) -> Self;

    /// SPR function
    fn spr(
        &mut self,
        edge1: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>),
        edge2: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>),
    ) {
        let pruned_tree = SPR::prune(self, edge1.1);
        SPR::graft(self, pruned_tree, edge2);
    }
}

pub trait NNI<'a>
where
    Self: RootedTree<'a> + Sized,
{
    fn nni(&mut self, parent_id: TreeNodeID<'a, Self>);
}

pub trait Reroot<'a>
where
    Self: RootedTree<'a> + Sized,
{
    fn reroot_at_node(&mut self, node_id: TreeNodeID<'a, Self>);
    fn reroot_at_edge(&mut self, edge: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>));
}

pub trait Balance<'a>: Clusters<'a> + SPR<'a> + Sized
where
    TreeNodeID<'a, Self>: Display + Debug + Hash + Clone + Ord,
{
    fn balance_subtree(&mut self);
}

pub trait Subtree<'a>: Ancestors<'a> + DFS<'a> + Sized
where
    TreeNodeID<'a, Self>: Display + Debug + Hash + Clone + Ord,
{
    fn induce_tree(
        &'a self,
        node_id_list: impl IntoIterator<
            Item = TreeNodeID<'a, Self>,
            IntoIter = impl ExactSizeIterator<Item = TreeNodeID<'a, Self>>,
        >,
    ) -> Self;

    fn subtree(&'a self, node_id: TreeNodeID<'a, Self>) -> Self;
}

pub trait RobinsonFoulds<'a>
where
    Self: RootedTree<'a> + Sized,
    TreeNodeID<'a, Self>: Display + Debug + Hash + Clone + Ord,
{
    fn rfs(&self, tree: Self) -> usize;
}

pub trait ClusterAffinity<'a>
where
    Self: RootedTree<'a> + Sized,
    TreeNodeID<'a, Self>: Display + Debug + Hash + Clone + Ord,
{
    fn ca(&self, tree: Self) -> usize;
}

pub trait WeightedRobinsonFoulds {
    fn wrfs(&self, tree: Self) -> usize;
}

pub trait ContractTree<'a>: EulerWalk<'a> + DFS<'a> {
    /// Contracts tree that from post_ord node_id iterator.
    fn contracted_tree_nodes_from_iter(
        &'a self,
        new_tree_root_id: TreeNodeID<'a, Self>,
        leaf_ids: &'a [TreeNodeID<'a, Self>],
        node_iter: impl Iterator<Item = TreeNodeID<'a, Self>>,
    ) -> impl Iterator<Item = Self::Node> {
        let mut node_map: HashMap<TreeNodeID<'a, Self>, Self::Node> =
            HashMap::from_iter(vec![(new_tree_root_id, self.get_lca(leaf_ids))]);
        let mut remove_list = vec![];
        node_iter
            .map(|x| self.get_node(x).cloned().unwrap())
            .for_each(|mut node| {
                match node.is_leaf() {
                    true => match leaf_ids.contains(&node.get_id()) {
                        true => {
                            node_map.insert(node.get_id(), node);
                        }
                        false => {}
                    },
                    false => {
                        let node_children_ids = node.get_children().collect_vec();
                        for child_id in &node_children_ids {
                            match node_map.contains_key(child_id) {
                                true => {}
                                false => node.remove_child(child_id),
                            }
                        }
                        let node_children_ids = node.get_children().collect_vec();
                        match node_children_ids.len() {
                            0 => {}
                            1 => {
                                // the node is a unifurcation
                                // node should be added to both node_map and remove_list
                                // if child of node is already in remove list, attach node children to node first
                                let child_node_id = node_children_ids[0];

                                match remove_list.contains(&child_node_id) {
                                    true => {
                                        node.remove_child(&child_node_id);
                                        let grandchildren_ids = node_map
                                            .get(&child_node_id)
                                            .unwrap()
                                            .get_children()
                                            .collect_vec();
                                        for grandchild_id in grandchildren_ids {
                                            node_map
                                                .get_mut(&grandchild_id)
                                                .unwrap()
                                                .set_parent(Some(node.get_id()));
                                            node.add_child(grandchild_id);
                                        }
                                    }
                                    false => {}
                                }
                                remove_list.push(node.get_id());
                                node_map.insert(node.get_id(), node);
                            }
                            _ => {
                                // node has multiple children
                                // for each child, suppress child if child is in remove list
                                node_children_ids.into_iter().for_each(|chid| {
                                    match remove_list.contains(&chid) {
                                        true => {
                                            // suppress chid
                                            // remove chid from node children
                                            // children of chid are node grandchildren
                                            // add grandchildren to node children
                                            // set grandchildren parent to node
                                            node.remove_child(&chid);
                                            let node_grandchildren = node_map
                                                .get(&chid)
                                                .unwrap()
                                                .get_children()
                                                .collect_vec();
                                            for grandchild in node_grandchildren {
                                                node.add_child(grandchild);
                                                node_map
                                                    .get_mut(&grandchild)
                                                    .unwrap()
                                                    .set_parent(Some(node.get_id()))
                                            }
                                        }
                                        false => {}
                                    }
                                });
                                if node.get_id() == new_tree_root_id {
                                    node.set_parent(None);
                                }
                                node_map.insert(node.get_id(), node);
                            }
                        };
                    }
                }
            });
        remove_list.into_iter().for_each(|x| {
            node_map.remove(&x);
        });
        node_map.into_values()
    }

    fn contracted_tree_nodes(
        &'a self,
        leaf_ids: &'a [TreeNodeID<'a, Self>],
    ) -> impl Iterator<Item = Self::Node> {
        let new_tree_root_id = self.get_lca_id(leaf_ids);
        let node_postord_iter = self.postord(new_tree_root_id);
        let mut node_map: HashMap<TreeNodeID<'a, Self>, Self::Node> =
            HashMap::from_iter(vec![(new_tree_root_id, self.get_lca(leaf_ids))]);
        let mut remove_list = vec![];
        node_postord_iter.for_each(|orig_node| {
            let mut node = orig_node.clone();
            match node.is_leaf() {
                true => match leaf_ids.contains(&node.get_id()) {
                    true => {
                        node_map.insert(node.get_id(), node);
                    }
                    false => {}
                },
                false => {
                    let node_children_ids = node.get_children().collect_vec();
                    for child_id in &node_children_ids {
                        match node_map.contains_key(child_id) {
                            true => {}
                            false => node.remove_child(child_id),
                        }
                    }
                    let node_children_ids = node.get_children().collect_vec();
                    match node_children_ids.len() {
                        0 => {}
                        1 => {
                            // the node is a unifurcation
                            // node should be added to both node_map and remove_list
                            // if child of node is already in remove list, attach node children to node first
                            let child_node_id = node_children_ids[0];

                            match remove_list.contains(&child_node_id) {
                                true => {
                                    node.remove_child(&child_node_id);
                                    let grandchildren_ids = node_map
                                        .get(&child_node_id)
                                        .unwrap()
                                        .get_children()
                                        .collect_vec();
                                    for grandchild_id in grandchildren_ids {
                                        node_map
                                            .get_mut(&grandchild_id)
                                            .unwrap()
                                            .set_parent(Some(node.get_id()));
                                        node.add_child(grandchild_id);
                                    }
                                }
                                false => {}
                            }
                            remove_list.push(node.get_id());
                            node_map.insert(node.get_id(), node);
                        }
                        _ => {
                            // node has multiple children
                            // for each child, suppress child if child is in remove list
                            node_children_ids.into_iter().for_each(|chid| {
                                match remove_list.contains(&chid) {
                                    true => {
                                        // suppress chid
                                        // remove chid from node children
                                        // children of chid are node grandchildren
                                        // add grandchildren to node children
                                        // set grandchildren parent to node
                                        node.remove_child(&chid);
                                        let node_grandchildren = node_map
                                            .get(&chid)
                                            .unwrap()
                                            .get_children()
                                            .collect_vec();
                                        for grandchild in node_grandchildren {
                                            node.add_child(grandchild);
                                            node_map
                                                .get_mut(&grandchild)
                                                .unwrap()
                                                .set_parent(Some(node.get_id()))
                                        }
                                    }
                                    false => {}
                                }
                            });
                            if node.get_id() == new_tree_root_id {
                                node.set_parent(None);
                            }
                            node_map.insert(node.get_id(), node.clone());
                        }
                    };
                }
            }
        });
        remove_list.into_iter().for_each(|x| {
            node_map.remove(&x);
        });
        node_map.into_values()
    }

    fn contract_tree(&'a self, leaf_ids: &'a [TreeNodeID<'a, Self>]) -> Self;

    fn contract_tree_from_iter(
        &self,
        leaf_ids: &'a [TreeNodeID<'a, Self>],
        node_iter: impl Iterator<Item = TreeNodeID<'a, Self>>,
    ) -> Self;
}

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
    // Returns zeta of leaf by taxa
    fn get_zeta_taxa(
        &'a self,
        taxa: &'a TreeNodeMeta<'a, Self>,
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
