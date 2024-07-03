use std::{collections::VecDeque, ops::Index};

// use fxhash::FxHashMap as HashMap;
use itertools::Itertools;

use crate::{node::simple_rnode::RootedTreeNode, tree::simple_rtree::RootedTree};

/// Trait describing post-order iteration of nodes in a tree
pub trait DFS<'a>
where
    Self: RootedTree<'a> + Sized,
{
    fn postord(
        &'a self,
        start_node: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl Iterator<Item = &'a Self::Node>;

    fn dfs(
        &'a self,
        start_node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl IntoIterator<Item = &'a Self::Node, IntoIter = impl ExactSizeIterator<Item = &'a Self::Node>>
    {
        let mut stack = VecDeque::from([self.get_node(start_node_id).unwrap()]);
        let mut out_vec = vec![];
        let mut visited = vec![];
        while let Some(x) = stack.pop_front() {
            match visited.contains(&x.get_id()) {
                false => {
                    visited.push(x.get_id());
                    out_vec.push(x);
                    for child_id in x.get_children().collect_vec().into_iter().rev() {
                        stack.push_front(self.get_node(child_id).unwrap());
                    }
                }
                true => {}
            };
        }
        out_vec
    }
}

pub trait BFS<'a>
where
    Self: RootedTree<'a> + Sized,
{
    fn bfs(
        &'a self,
        start_node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl Iterator<Item = &'a Self::Node>;
}

pub trait PreOrder<'a>
where
    Self: RootedTree<'a> + Sized,
{
    fn preord(
        &'a self,
        start_node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl IntoIterator<Item = &'a Self::Node, IntoIter = impl ExactSizeIterator<Item = &'a Self::Node>>
    {
        let mut stack = VecDeque::from([self.get_node(start_node_id).unwrap()]);
        let mut out_vec = vec![];
        let mut visited = vec![];
        while let Some(x) = stack.pop_front() {
            match visited.contains(&x.get_id()) {
                false => {
                    visited.push(x.get_id());
                    out_vec.push(x);
                    for child_id in x.get_children().collect_vec().into_iter().rev() {
                        stack.push_front(self.get_node(child_id).unwrap());
                    }
                }
                true => {}
            };
        }
        out_vec
    }
}

pub trait Ancestors<'a>
where
    Self: RootedTree<'a> + Sized,
{
    fn root_to_node(
        &'a self,
        start_node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl IntoIterator<Item = &'a Self::Node, IntoIter = impl ExactSizeIterator<Item = &'a Self::Node>>
    {
        let mut stack = VecDeque::from([self.get_node(start_node_id).unwrap()]);
        while let Some(x) = stack.pop_front() {
            stack.push_front(x);
            match x.get_parent() {
                Some(pid) => {
                    stack.push_front(self.get_node(pid).unwrap());
                }
                None => {
                    break;
                }
            }
        }
        stack
    }

    fn node_to_root(
        &'a self,
        start_node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl IntoIterator<Item = &'a Self::Node, IntoIter = impl ExactSizeIterator<Item = &'a Self::Node>>
    {
        let mut stack = VecDeque::from([self.get_node(start_node_id).unwrap()]);
        while let Some(x) = stack.pop_front() {
            stack.push_back(x);
            match x.get_parent() {
                Some(pid) => {
                    stack.push_front(self.get_node(pid).unwrap());
                }
                None => {
                    break;
                }
            }
        }
        stack
    }

    fn depth(
        &'a self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> usize {
        self.node_to_root(node_id).into_iter().len() - 1
    }
}

pub trait EulerWalk<'a>
where
    Self: RootedTree<'a> + Sized,
{
    fn precompute_walk(&mut self);

    fn get_precomputed_walk(
        &self,
    ) -> Option<&Vec<<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID>>;

    fn precompute_fai(&mut self);

    fn get_precomputed_fai(
        &'a self,
    ) -> Option<
        impl Index<<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, Output = Option<usize>>,
    >;

    fn precompute_da(&mut self);

    fn get_precomputed_da(&self) -> Option<&Vec<usize>>;

    fn precompute_constant_time_lca(&mut self) {
        self.precompute_walk();
        self.precompute_da();
        self.precompute_fai();
    }

    fn euler_walk(
        &'a self,
        start_node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl ExactSizeIterator<Item = &'a Self::Node> {
        let mut stack = VecDeque::from([self.get_node(start_node_id).unwrap()]);
        let mut visited = vec![];
        let mut out_vec = vec![];
        while let Some(node) = stack.pop_front() {
            match visited.contains(&node.get_id()) {
                true => {
                    if let Some(parent_id) = node.get_parent() {
                        out_vec.push(self.get_node(parent_id).unwrap())
                    }
                }
                false => {
                    visited.push(node.get_id());
                    out_vec.push(node);
                    stack.push_front(node);
                    for child_id in node.get_children().rev() {
                        stack.push_front(self.get_node(child_id).unwrap())
                    }
                }
            }
        }
        out_vec.into_iter()
    }

    fn is_euler_precomputed(&self) -> bool {
        self.get_precomputed_walk().is_some()
    }

    /// returns the first appearance index of each node in the euler tour
    fn first_appearance(
        &self,
    ) -> impl Index<<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, Output = Option<usize>>;
    // {
    //     let mut fa: HashMap<<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, usize> = [].into_iter().collect::<HashMap<_, _>>();
    //     match self.get_precomputed_walk() {
    //         Some(walk) => {
    //             for (idx, node_id) in walk.iter().enumerate() {
    //                 if fa.contains_key(node_id) {
    //                 } else {
    //                     fa.insert(*node_id, idx);
    //                 }
    //             }
    //         }
    //         None => {
    //             let walk = self.euler_walk(self.get_root_id()).map(|x| x.get_id());
    //             for (idx, node_id) in walk.enumerate() {
    //                 if let std::collections::hash_map::Entry::Vacant(e) = fa.entry(node_id) {
    //                     e.insert(idx);
    //                 }
    //             }
    //         }
    //     }

    //     fa
    // }

    fn is_fai_precomputed(&'a self) -> bool {
        self.get_precomputed_fai().is_some()
    }

    fn get_fa_index(
        &'a self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> usize {
        match self.get_precomputed_fai() {
            Some(fai) => fai[node_id].unwrap(),
            None => self.first_appearance()[node_id].unwrap(),
        }
    }

    /// depth array for nodes
    fn depth_array(&'a self) -> Vec<usize> {
        let da = match self.get_precomputed_walk() {
            Some(walk) => walk
                .iter()
                .map(|x| RootedTree::get_node_depth(self, *x))
                .collect_vec(),
            None => self
                .euler_walk(self.get_root_id())
                .map(|x| RootedTree::get_node_depth(self, x.get_id()))
                .collect_vec(),
        };
        da
    }

    fn is_da_precomputed(&self) -> bool {
        self.get_precomputed_da().is_some()
    }

    fn get_node_depth(
        &'a self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> usize {
        match self.get_precomputed_da() {
            Some(da) => da[self.get_fa_index(node_id)],
            None => RootedTree::get_node_depth(self, node_id),
        }
    }

    fn get_euler_slice(
        &'a self,
        start: usize,
        end: usize,
    ) -> Vec<<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID> {
        match self.get_precomputed_walk() {
            Some(walk) => walk[start..end].to_vec(),
            None => self
                .euler_walk(self.get_root_id())
                .map(|x| x.get_id())
                .collect_vec()[start..end]
                .to_vec(),
        }
    }

    fn get_da_slice(&'a self, start: usize, end: usize) -> Vec<usize> {
        match self.get_precomputed_da() {
            Some(da) => da[start..end].to_vec(),
            None => self.depth_array()[start..end].to_vec(),
        }
    }

    fn get_euler_pos(
        &'a self,
        pos: usize,
    ) -> <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID {
        match self.get_precomputed_walk() {
            Some(walk) => walk[pos],
            None => self
                .euler_walk(self.get_root_id())
                .map(|x| x.get_id())
                .nth(pos)
                .unwrap(),
        }
    }

    /// constant time lca query
    fn get_lca_id(
        &'a self,
        node_id_vec: &[<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID],
    ) -> <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID {
        if node_id_vec.len() == 1 {
            return node_id_vec[0];
        }
        let min_pos = node_id_vec
            .iter()
            .map(|x| self.get_fa_index(*x))
            .min()
            .unwrap();
        let max_pos = node_id_vec
            .iter()
            .map(|x| self.get_fa_index(*x))
            .max()
            .unwrap();

        let depth_subarray_min_value = self
            .get_da_slice(min_pos, max_pos)
            .into_iter()
            .min()
            .unwrap();
        let depth_subarray_min_pos = self
            .get_da_slice(min_pos, max_pos)
            .into_iter()
            .position(|x| x == depth_subarray_min_value)
            .unwrap();
        self.get_euler_pos(min_pos + depth_subarray_min_pos)
    }

    fn get_lca(
        &'a self,
        node_id_vec: &'a [<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID],
    ) -> Self::Node {
        self.get_node(self.get_lca_id(node_id_vec))
            .cloned()
            .unwrap()
    }
}

pub trait Clusters<'a>: DFS<'a> + BFS<'a> + Sized {
    fn get_cluster(
        &'a self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl ExactSizeIterator<Item = &'a Self::Node> {
        self.dfs(node_id)
            .into_iter()
            .filter(|x| x.is_leaf())
            .collect_vec()
            .into_iter()
    }
    fn get_cluster_ids(
        &'a self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl ExactSizeIterator<Item = <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID>
    {
        self.get_cluster(node_id).map(|x| x.get_id())
    }
    fn get_cluster_size(
        &'a self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> usize {
        self.get_cluster(node_id).len()
    }

    fn get_bipartition(
        &'a self,
        edge: (
            <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
            <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
        ),
    ) -> (
        impl ExactSizeIterator<Item = &'a Self::Node>,
        impl ExactSizeIterator<Item = &'a Self::Node>,
    ) {
        let c2 = self.get_cluster(edge.1);
        let c2_ids = self.get_cluster(edge.1).map(|x| x.get_id()).collect_vec();
        let c1 = self
            .get_cluster(edge.0)
            .filter(|x| !c2_ids.contains(&x.get_id()))
            .collect_vec();
        (c1.into_iter(), c2.into_iter())
    }

    fn get_median_node_id_for_leaves(
        &'a self,
        taxa_set: impl ExactSizeIterator<
            Item = <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
        >,
    ) -> <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID {
        let mut median_node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID =
            self.get_root_id();
        let leaf_ids = taxa_set.collect_vec();
        loop {
            median_node_id = self
                .get_node_children_ids(median_node_id)
                // .filter(|x| !self.is_leaf(x))
                .max_by(|x, y| {
                    let x_cluster = self
                        .get_cluster_ids(*x)
                        .filter(|nids| leaf_ids.contains(nids))
                        .collect_vec();
                    let y_cluster = self
                        .get_cluster_ids(*y)
                        .filter(|nids| leaf_ids.contains(nids))
                        .collect_vec();
                    x_cluster.len().cmp(&y_cluster.len())
                })
                .unwrap();
            let num_leaves = self
                .get_cluster_ids(median_node_id)
                .filter(|nids| leaf_ids.contains(nids))
                .collect_vec()
                .len();
            if (num_leaves) <= (leaf_ids.len() / 2) {
                break;
            }
        }
        median_node_id
    }

    fn get_median_node_for_leaves(
        &'a self,
        taxa_set: impl ExactSizeIterator<
            Item = <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
        >,
    ) -> Self::Node {
        self.get_node(self.get_median_node_id_for_leaves(taxa_set))
            .cloned()
            .unwrap()
    }

    fn get_median_node(&'a self) -> Self::Node {
        let leaves = self.get_leaves().map(|x| x.get_id());
        self.get_median_node_for_leaves(leaves)
    }

    fn get_median_node_id(&'a self) -> <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID {
        self.get_median_node().get_id()
    }
}
