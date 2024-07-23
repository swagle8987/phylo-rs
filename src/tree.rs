/// Module with traits and structs for distance computation
pub mod distances;
/// Module with traits and structs for tree encoding
pub mod io;
/// Module with traits and structs for tree operations
pub mod ops;
/// Module with traits and structs for general tree traits
pub mod simple_rtree;
/// Module with traits and structs for tree simulation
pub mod simulation;

use fxhash::FxHashMap as HashMap;
use simulation::{Uniform, Yule};
use std::ops::Index;

use anyhow::Result;
use itertools::Itertools;
use rand::prelude::IteratorRandom;

use crate::prelude::*;

use crate::iter::{BFSIterator, DFSPostOrderIterator};
use crate::node::{Node, NodeID};
use vers_vecs::BinaryRmq;

/// Arena memory-managed tree struct
#[derive(Debug, Clone)]
pub struct SimpleRootedTree {
    /// Root NodeID
    root: NodeID,
    /// Nodes of the tree
    nodes: Vec<Option<Node>>,
    /// Index of nodes by taxa
    taxa_node_id_map: HashMap<String, NodeID>,
    /// Field to hold precomputed euler tour for constant-time LCA queries
    precomputed_euler: Option<Vec<NodeID>>,
    /// Field to hold precomputed first-appearance for constant-time LCA queries
    precomputed_fai: Option<Vec<Option<usize>>>,
    /// Field to hold precomputed depth-array for constant-time LCA queries
    precomputed_da: Option<Vec<usize>>,
    /// Field to hold precomputed range-minimum-query for constant-time LCA queries
    precomputed_rmq: Option<BinaryRmq>,
}

impl SimpleRootedTree {
    /// Creates new empty tree
    pub fn new(root_id: NodeID) -> Self {
        let root_node = Node::new(root_id);
        let mut nodes = vec![None; root_id + 1];
        nodes[root_id] = Some(root_node);
        SimpleRootedTree {
            root: root_id,
            nodes,
            precomputed_euler: None,
            taxa_node_id_map: [].into_iter().collect::<HashMap<_, _>>(),
            precomputed_fai: None,
            precomputed_da: None,
            precomputed_rmq: None,
        }
    }

    /// Returns new empty tree
    pub fn next_id(&self) -> usize {
        match &self.nodes.iter().position(|r| r.is_none()) {
            Some(x) => *x,
            None => self.nodes.len(),
        }
    }

    /// Creates new node with next NodeID
    pub fn next_node(&self) -> Node {
        Node::new(self.next_id())
    }
}

impl<'a> RootedTree<'a> for SimpleRootedTree {
    type Node = Node;

    /// Returns reference to node by ID
    fn get_node(&'a self, node_id: TreeNodeID<'a, Self>) -> Option<&'a Node> {
        self.nodes[node_id].as_ref()
    }

    fn get_node_mut(&mut self, node_id: TreeNodeID<'a, Self>) -> Option<&mut Node> {
        self.nodes[node_id].as_mut()
    }

    fn get_node_ids(&self) -> impl Iterator<Item = TreeNodeID<'a, Self>> {
        (0..self.nodes.len()).filter(|x| self.nodes[*x].is_some())
    }

    fn get_nodes(&'a self) -> impl ExactSizeIterator<Item = &'a Node> {
        let node_ids = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.is_some())
            .map(|(id, _)| id)
            .collect_vec();
        node_ids
            .into_iter()
            .map(move |id| self.nodes[id].as_ref().unwrap())
    }

    fn get_nodes_mut(&'a mut self) -> impl Iterator<Item = &'a mut Self::Node> {
        self.nodes.iter_mut().filter_map(|x| x.as_mut())
    }

    fn get_node_parent_id(&self, node_id: TreeNodeID<'a, Self>) -> Option<TreeNodeID<'a, Self>> {
        self.get_node(node_id).unwrap().get_parent()
    }

    fn get_node_children_ids(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl ExactSizeIterator<Item = TreeNodeID<'a, Self>> {
        self.get_node(node_id)
            .unwrap()
            .get_children()
            .collect_vec()
            .into_iter()
    }

    /// Returns reference to node by ID
    fn set_node(&mut self, node: Node) {
        let node_id = node.get_id();
        match node.get_taxa() {
            None => {}
            Some(taxa) => {
                self.taxa_node_id_map
                    .insert(taxa.to_string(), node.get_id());
            }
        };
        match self.nodes.len() > node_id {
            true => self.nodes[node_id] = Some(node),
            false => {
                let new_len = node.get_id() - self.nodes.len();
                self.nodes.extend((0..new_len + 1).map(|_| None));
                self.nodes[node_id] = Some(node);
            }
        };
    }

    fn set_nodes(
        &mut self,
        node_list: impl IntoIterator<
            Item = Self::Node,
            IntoIter = impl ExactSizeIterator<Item = Self::Node>,
        >,
    ) {
        for node in node_list.into_iter() {
            self.set_node(node);
        }
    }

    fn add_child(&mut self, parent_id: TreeNodeID<'a, Self>, child: Node) {
        let new_child_id = child.get_id();
        self.set_node(child);
        self.get_node_mut(parent_id)
            .unwrap()
            .add_child(new_child_id);
        self.get_node_mut(new_child_id)
            .unwrap()
            .set_parent(Some(parent_id));
    }

    /// Get root node ID
    fn get_root_id(&self) -> TreeNodeID<'a, Self> {
        self.root
    }

    fn get_root_mut(&'a mut self) -> &'a mut Self::Node {
        self.get_node_mut(self.get_root_id()).unwrap()
    }

    fn set_root(&mut self, node_id: TreeNodeID<'a, Self>) {
        self.root = node_id;
    }

    fn remove_node(&mut self, node_id: TreeNodeID<'a, Self>) -> Option<Node> {
        if let Some(pid) = self.get_node_parent_id(node_id) {
            self.get_node_mut(pid).unwrap().remove_child(&node_id)
        }
        self.nodes[node_id].take()
    }

    fn delete_node(&mut self, node_id: TreeNodeID<'a, Self>) {
        let _ = self.nodes[node_id].take();
    }

    fn contains_node(&self, node_id: TreeNodeID<'a, Self>) -> bool {
        self.nodes[node_id].is_some()
    }

    fn delete_edge(&mut self, parent_id: TreeNodeID<'a, Self>, child_id: TreeNodeID<'a, Self>) {
        self.get_node_mut(parent_id)
            .unwrap()
            .remove_child(&child_id);
        self.get_node_mut(child_id).unwrap().set_parent(None);
    }

    fn clean(&mut self) {
        let node_iter = self.get_nodes().cloned().collect_vec();
        for node in &node_iter {
            // remove root with only one child
            let node_id = node.get_id();
            if node.get_id() == self.get_root_id() && node.degree() < 2 {
                let new_root = self.get_root().get_children().next().unwrap();
                self.set_root(new_root);
                self.get_node_mut(self.root).unwrap().set_parent(None);
                self.remove_node(node_id);
            }
            // remove nodes with only one child
            else if !node.is_leaf() && node.get_parent().is_some() && node.degree() < 3 {
                let parent_id = self.get_node_parent_id(node_id);
                let child_id = node.get_children().next().unwrap();
                self.get_node_mut(child_id).unwrap().set_parent(parent_id);
                self.get_node_mut(parent_id.unwrap())
                    .unwrap()
                    .add_child(child_id);
                self.remove_node(node.get_id());
            }
            // Removing dangling references to pruned children
            for chid in node.get_children() {
                if !self.get_nodes().map(|x| x.get_id()).contains(&chid) {
                    self.get_node_mut(node_id).unwrap().remove_child(&chid);
                }
            }
        }
    }

    fn clear(&mut self) {
        let root_node = self.get_root().clone();
        let root_node_id = root_node.get_id();
        self.nodes = vec![None; root_node_id + 1];
        self.nodes[root_node_id] = Some(root_node);
        self.taxa_node_id_map.clear();
    }

    fn split_edge(
        &'a mut self,
        edge: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>),
        node: Self::Node,
    ) {
        let p_id = edge.0;
        let c_id = edge.1;
        let n_id = node.get_id();
        self.set_node(node);
        self.get_node_mut(p_id).unwrap().remove_child(&c_id);
        self.set_child(p_id, n_id);
        self.set_child(n_id, c_id);
    }

    fn add_sibling(
        &'a mut self,
        node_id: TreeNodeID<'a, Self>,
        split_node: Self::Node,
        sibling_node: Self::Node,
    ) {
        let node_parent_id = self.get_node_parent_id(node_id).unwrap();
        let split_node_id = split_node.get_id();
        self.split_edge((node_parent_id, node_id), split_node);
        self.add_child(split_node_id, sibling_node);
    }

    fn set_child(&mut self, parent_id: TreeNodeID<'a, Self>, child_id: TreeNodeID<'a, Self>) {
        self.get_node_mut(parent_id).unwrap().add_child(child_id);
        self.get_node_mut(child_id)
            .unwrap()
            .set_parent(Some(parent_id));
    }

    fn remove_children(
        &'a mut self,
        parent_id: TreeNodeID<'a, Self>,
        child_ids: impl Iterator<Item=TreeNodeID<'a, Self>>,
    ) {
        for child_id in child_ids {
            self.get_node_mut(parent_id)
                .unwrap()
                .remove_child(&child_id);
        }
    }

    fn remove_all_children(&'a mut self, node_id: TreeNodeID<'a, Self>) {
        let node_children_ids = self.get_node_children_ids(node_id).collect_vec().into_iter();
        self.remove_children(node_id, node_children_ids);
    }

    fn is_leaf(&self, node_id: TreeNodeID<'a, Self>) -> bool {
        self.get_node(node_id).unwrap().is_leaf()
    }

}

impl<'a> RootedMetaTree<'a> for SimpleRootedTree {
    fn get_taxa_node_id(&self, taxa: &TreeNodeMeta<'a, Self>) -> Option<TreeNodeID<'a, Self>> {
        self.taxa_node_id_map.get(taxa).cloned()
    }

    fn get_taxa_node(&self, taxa: &TreeNodeMeta<'a, Self>) -> Option<&Self::Node> {
        match self.taxa_node_id_map.get(taxa) {
            None => None,
            Some(node_id) => self.get_node(*node_id),
        }
    }

    fn set_node_taxa(
        &mut self,
        node_id: TreeNodeID<'a, Self>,
        taxa: Option<TreeNodeMeta<'a, Self>>,
    ) {
        self.get_node_mut(node_id).unwrap().set_taxa(taxa.clone());
        if let Some(t) = taxa {
            self.taxa_node_id_map.insert(t, node_id);
        }
    }

    fn num_taxa(&self) -> usize {
        self.taxa_node_id_map.len()
    }

    fn get_taxa_space(&'a self) -> impl ExactSizeIterator<Item = &'a TreeNodeMeta<'a, Self>> {
        self.taxa_node_id_map.keys()
    }

    fn get_node_taxa_cloned(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> Option<TreeNodeMeta<'a, Self>> {
        self.get_node(node_id).unwrap().get_taxa().cloned()
    }

}

impl<'a> Yule<'a> for SimpleRootedTree{
    fn yule(num_taxa: usize) -> Result<SimpleRootedTree> {
        let mut tree = SimpleRootedTree::new(0);
        if num_taxa < 3 {
            return Ok(tree);
        }
        let new_node = Node::new(1);
        tree.add_child(0, new_node);
        tree.set_node_taxa(1, Some("0".to_string()));
        let new_node = Node::new(2);
        tree.add_child(0, new_node);
        tree.set_node_taxa(2, Some("1".to_string()));
        if num_taxa < 4 {
            return Ok(tree);
        }
        let mut current_leaf_ids = vec![1, 2];
        for i in 2..num_taxa {
            let rand_leaf_id = current_leaf_ids
                .iter()
                .choose(&mut rand::thread_rng())
                .unwrap();
            let rand_leaf_parent_id = tree.get_node_parent_id(*rand_leaf_id).unwrap();
            let split_node = Node::new(tree.next_id());
            let split_node_id = split_node.get_id();
            tree.split_edge((rand_leaf_parent_id, *rand_leaf_id), split_node);
            let new_leaf = Node::new(tree.next_id());
            let new_leaf_id = new_leaf.get_id();
            tree.add_child(split_node_id, new_leaf);
            tree.set_node_taxa(new_leaf_id, Some(i.to_string()));
            current_leaf_ids.push(new_leaf_id);
        }
        Ok(tree)
    }
}

impl<'a> Uniform<'a> for SimpleRootedTree{
    fn unif(num_taxa: usize) -> Result<SimpleRootedTree> {
        let mut tree = SimpleRootedTree::new(0);
        if num_taxa < 3 {
            return Ok(tree);
        }
        let new_node = Node::new(1);
        tree.add_child(0, new_node);
        tree.set_node_taxa(1, Some("0".to_string()));
        let new_node = Node::new(2);
        tree.add_child(0, new_node);
        tree.set_node_taxa(2, Some("1".to_string()));
        if num_taxa < 3 {
            return Ok(tree);
        }
        let mut current_node_ids = vec![1, 2];
        for i in 1..num_taxa {
            let rand_leaf_id = *current_node_ids
                .iter()
                .choose(&mut rand::thread_rng())
                .unwrap();
            let rand_leaf_parent_id = tree.get_node_parent_id(rand_leaf_id).unwrap();
            let split_node = Node::new(tree.next_id());
            let split_node_id = split_node.get_id();
            current_node_ids.push(split_node_id);
            tree.split_edge((rand_leaf_parent_id, rand_leaf_id), split_node);
            let mut new_leaf = Node::new(tree.next_id());
            new_leaf.set_taxa(Some(i.to_string()));
            current_node_ids.push(new_leaf.get_id());
            tree.add_child(split_node_id, new_leaf)
        }
        Ok(tree)
    }

}

impl<'a> RootedWeightedTree<'a> for SimpleRootedTree {
    fn unweight(&'a mut self) {
        self.nodes
            .iter_mut()
            .filter(|x| x.is_none())
            .for_each(|x| x.as_mut().unwrap().unweight());
    }
}

impl<'a> PathFunction<'a> for SimpleRootedTree {
    fn get_zeta(&self, node_id: NodeID) -> Option<f32> {
        self.get_node(node_id)
            .unwrap_or_else(|| panic!("Node with ID {} not found in tree", node_id))
            .get_zeta()
    }
    fn set_zeta(&'a mut self, zeta_func: fn(&Self, NodeID) -> TreeNodeZeta<'a, Self>) {
        let zetas = (0..self.nodes.len())
            .filter(|x| self.nodes[*x].is_some())
            .map(|node_id| (node_id, Some(zeta_func(self, node_id))))
            .collect_vec();
        for (node_id, zeta) in zetas {
            self.nodes[node_id].as_mut().unwrap().set_zeta(zeta);
        }
    }
    fn is_zeta_set(
        &self,
        node_id: <<Self as RootedTree<'_>>::Node as RootedTreeNode>::NodeID,
    ) -> bool {
        self.get_node(node_id).unwrap().is_zeta_set()
    }
    fn set_node_zeta(
        &mut self,
        node_id: TreeNodeID<'a, Self>,
        zeta: Option<TreeNodeZeta<'a, Self>>,
    ) {
        self.nodes[node_id].as_mut().unwrap().set_zeta(zeta);
    }

    fn is_all_zeta_set(&self) -> bool {
        !self.get_nodes().any(|x| !x.is_zeta_set())
    }

}

impl<'a> Ancestors<'a> for SimpleRootedTree {}

impl<'a> Subtree<'a> for SimpleRootedTree {
    fn induce_tree(
        &'a self,
        node_id_list: impl IntoIterator<
            Item = TreeNodeID<'a, Self>,
            IntoIter = impl ExactSizeIterator<Item = TreeNodeID<'a, Self>>,
        >,
    ) -> Self {
        let mut subtree = self.clone();
        subtree.clear();
        for node_id in node_id_list.into_iter() {
            let ancestors = self
                .root_to_node(node_id)
                .into_iter()
                .cloned()
                .collect_vec();
            subtree.set_nodes(ancestors);
        }
        subtree.clean();
        subtree.clone()
    }

    fn subtree(&'a self, node_id: TreeNodeID<'a, Self>) -> Self {
        let mut subtree = self.clone();
        let dfs = self.dfs(node_id).into_iter().cloned().collect_vec();
        subtree.set_nodes(dfs);
        subtree
    }
}

impl<'a> PreOrder<'a> for SimpleRootedTree {}

impl<'a> DFS<'a> for SimpleRootedTree {
    fn postord_ids(&self, start_node: TreeNodeID<'a, Self>) -> impl Iterator<Item = TreeNodeID<'a, Self>> {
        DFSPostOrderIterator::new(self, start_node).map(|x| x.get_id())
    }

    fn postord_nodes(&'a self, start_node: TreeNodeID<'a, Self>) -> impl Iterator<Item = &'a Self::Node> {
        DFSPostOrderIterator::new(self, start_node)
    }
}

impl<'a> BFS<'a> for SimpleRootedTree {
    fn bfs_nodes(&'a self, start_node_id: TreeNodeID<'a, Self>) -> impl Iterator<Item = &'a Self::Node> {
        BFSIterator::new(self, start_node_id)
    }

    fn bfs_ids(&self, start_node_id: TreeNodeID<'a, Self>) -> impl Iterator<Item = TreeNodeID<'a, Self>> {
        BFSIterator::new(self, start_node_id).map(|x| x.get_id())
    }
}

impl<'a> ContractTree<'a> for SimpleRootedTree {
    fn contract_tree(&self, leaf_ids: &[TreeNodeID<'a, Self>]) -> Self {
        let new_tree_root_id = self.get_lca_id(leaf_ids);
        let new_nodes = self.contracted_tree_nodes(leaf_ids).collect_vec();
        let mut new_tree = self.clone();
        new_tree.set_root(new_tree_root_id);
        new_tree.clear();
        new_tree.set_nodes(new_nodes);
        new_tree
    }

    fn contract_tree_from_iter(
        &self,
        leaf_ids: &[TreeNodeID<'a, Self>],
        node_iter: impl Iterator<Item = TreeNodeID<'a, Self>>,
    ) -> Self {
        let new_tree_root_id = self.get_lca_id(leaf_ids);
        let new_nodes = self
            .contracted_tree_nodes_from_iter(new_tree_root_id, leaf_ids, node_iter)
            .collect_vec();
        let mut new_tree = self.clone();
        new_tree.set_root(new_tree_root_id);
        new_tree.clear();
        new_tree.set_nodes(new_nodes);
        new_tree
    }
}

impl<'a> EulerWalk<'a> for SimpleRootedTree {
    fn precompute_fai(&mut self) {
        // todo!()
        let max_id = self.get_node_ids().max().unwrap();
        let mut index = vec![None; max_id + 1];
        match self.get_precomputed_walk() {
            Some(walk) => {
                for node_id in self.get_node_ids() {
                    index[node_id] = Some(walk.iter().position(|x| x == &node_id).unwrap());
                }
            }
            None => {
                let walk = self
                    .euler_walk_ids(self.get_root_id())
                    .collect_vec();
                for node_id in self.get_node_ids() {
                    index[node_id] = Some(walk.iter().position(|x| x == &node_id).unwrap());
                }
            }
        }
        let mut fai_vec = vec![None; max_id + 1];
        for id in self.get_node_ids() {
            fai_vec[id] = index[id];
        }
        self.precomputed_fai = Some(fai_vec);
    }

    fn precompute_da(&mut self) {
        let da = self.depth_array();
        let rmq = BinaryRmq::from_vec(da.iter().map(|x| *x as u64).collect_vec());
        self.precomputed_rmq = Some(rmq);

        self.precomputed_da = Some(self.depth_array());
    }

    fn precompute_walk(&mut self) {
        self.precomputed_euler = Some(
            self.euler_walk_ids(self.get_root_id())
                .collect_vec(),
        );
    }

    fn get_precomputed_walk(
        &self,
    ) -> Option<&Vec<<<Self as RootedTree>::Node as RootedTreeNode>::NodeID>> {
        self.precomputed_euler.as_ref()
    }

    fn precompute_constant_time_lca(&mut self) {
        self.precompute_walk();
        self.precompute_da();
        self.precompute_fai();
    }

    fn get_precomputed_fai(
        &self,
    ) -> Option<impl Index<TreeNodeID<'a, Self>, Output = Option<usize>>> {
        self.precomputed_fai.clone()
    }

    fn get_precomputed_da(&self) -> Option<&Vec<usize>> {
        self.precomputed_da.as_ref()
    }

    fn is_euler_precomputed(&self) -> bool {
        self.precomputed_euler.is_some()
    }

    #[allow(refining_impl_trait)]
    fn first_appearance(&self) -> impl Index<TreeNodeID<'a, Self>, Output = Option<usize>> {
        let max_id = self.get_node_ids().max().unwrap();
        let mut fa = vec![None; max_id + 1];
        match self.get_precomputed_walk() {
            Some(walk) => {
                for node_id in self.get_node_ids() {
                    fa[node_id] = Some(walk.iter().position(|x| x == &node_id).unwrap());
                }
            }
            None => {
                let walk = self
                    .euler_walk_ids(self.get_root_id())
                    .collect_vec();
                for node_id in self.get_node_ids() {
                    fa[node_id] = Some(walk.iter().position(|x| x == &node_id).unwrap());
                }
            }
        }

        fa
    }

    fn get_fa_index(&self, node_id: TreeNodeID<'a, Self>) -> usize {
        match &self.precomputed_fai {
            Some(fai) => fai[node_id].unwrap(),
            None => self.first_appearance()[node_id].unwrap(),
        }
    }

    fn get_lca_id(&self, node_id_vec: &[TreeNodeID<'a, Self>]) -> TreeNodeID<'a, Self> {
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

        if min_pos == max_pos {
            return min_pos;
        }

        match self.precomputed_rmq.as_ref() {
            Some(dp) => self.get_euler_pos(dp.range_min(min_pos, max_pos)),
            None => {
                let da = self.depth_array();
                let rmq = BinaryRmq::from_vec(da.iter().map(|x| *x as u64).collect_vec());
                self.get_euler_pos(rmq.range_min(min_pos, max_pos))
            }
        }
    }
}

impl<'a> Clusters<'a> for SimpleRootedTree {
    fn get_median_node_id(&self) -> TreeNodeID<'a, Self> {
        self.get_median_node().get_id()
    }

    fn get_cluster_ids(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl ExactSizeIterator<Item = TreeNodeID<'a, Self>> {
        self.get_cluster(node_id).map(move |x| x.get_id())
    }

}

impl<'a> Newick<'a> for SimpleRootedTree {
    fn from_newick(newick_str: &[u8]) -> Result<Self> {
        let mut tree = SimpleRootedTree::new(0);
        let mut stack: Vec<TreeNodeID<'a, Self>> = Vec::new();
        let mut context: TreeNodeID<'a, Self> = tree.get_root_id();
        let mut taxa_str = String::new();
        let mut decimal_str: String = String::new();
        let mut str_ptr: usize = 0;
        let newick_string = String::from_utf8(newick_str.to_vec())
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<char>>();
        while str_ptr < newick_string.len() {
            match newick_string[str_ptr] {
                '(' => {
                    stack.push(context);
                    let new_node = Node::new(tree.next_id());
                    context = new_node.get_id();
                    tree.set_node(new_node);
                    str_ptr += 1;
                }
                ')' | ',' => {
                    // last context id
                    let last_context = stack.last().ok_or(NewickErr("Newick string ended abruptly!".to_string()))?;
                    // add current context as a child to last context
                    tree.set_child(*last_context, context);
                    tree.set_edge_weight(
                        (*last_context, context),
                        decimal_str.parse::<TreeNodeWeight<'a, Self>>().ok(),
                    );
                    if !taxa_str.is_empty() {
                        tree.set_node_taxa(context, Some(taxa_str.to_string()));
                    }
                    // we clear the strings
                    taxa_str.clear();
                    decimal_str.clear();

                    match newick_string[str_ptr] {
                        ',' => {
                            let new_node = Node::new(tree.next_id());
                            context = new_node.get_id();
                            tree.set_node(new_node);
                            str_ptr += 1;
                        }
                        _ => {
                            context = stack.pop().ok_or(NewickErr("Newick string ended abruptly!".to_string()))?;
                            str_ptr += 1;
                        }
                    }
                }
                ';' => {
                    if !taxa_str.is_empty() {
                        tree.set_node_taxa(context, Some(taxa_str));
                    }
                    break;
                }
                ':' => {
                    // if the current context had a weight
                    if newick_string[str_ptr] == ':' {
                        str_ptr += 1;
                        while newick_string[str_ptr].is_ascii_digit()
                            || newick_string[str_ptr] == '.'
                        {
                            decimal_str.push(newick_string[str_ptr]);
                            str_ptr += 1;
                        }
                    }
                }
                _ => {
                    // push taxa characters into taxa string
                    while newick_string[str_ptr] != ':'
                        && newick_string[str_ptr] != ')'
                        && newick_string[str_ptr] != ','
                        && newick_string[str_ptr] != '('
                        && newick_string[str_ptr] != ';'
                    {
                        taxa_str.push(newick_string[str_ptr]);
                        str_ptr += 1;
                    }
                }
            }
        }
        Ok(tree)
    }

    fn subtree_to_newick(&self, node_id: TreeNodeID<'a, Self>) -> impl std::fmt::Display {
        let node = self.get_node(node_id).unwrap();
        let mut tmp = String::new();
        if node.get_children().len() != 0 {
            if node.get_children().len() > 1 {
                tmp.push('(');
            }
            for child_id in node.get_children() {
                let child_str = format!("{},", self.subtree_to_newick(child_id));
                tmp.push_str(&child_str);
            }
            tmp.pop();
            if node.get_children().collect_vec().len() > 1 {
                tmp.push(')');
            }
        }
        tmp.push_str(node.get_taxa().unwrap_or(&"".to_string()));
        if let Some(w)=node.get_weight(){
            tmp.push_str(":");
            tmp.push_str(&w.to_string());
        }
        tmp
    }
}

impl<'a> Nexus<'a> for SimpleRootedTree {}

impl<'a> SPR<'a> for SimpleRootedTree {
    fn graft(&mut self, tree: Self, edge: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>)) {
        let new_node = self.next_node();
        let new_node_id = dbg!(new_node.get_id());
        for node in tree.dfs(tree.get_root_id()) {
            self.set_node(node.clone());
        }
        self.split_edge(edge, new_node);
        self.set_child(dbg!(new_node_id), dbg!(tree.get_root_id()));
    }
    fn prune(&mut self, node_id: TreeNodeID<'a, Self>) -> Self {
        let mut pruned_tree = SimpleRootedTree::new(node_id);
        let p_id = self.get_node_parent_id(node_id).unwrap();
        self.get_node_mut(p_id).unwrap().remove_child(&node_id);
        pruned_tree
            .get_node_mut(pruned_tree.get_root_id())
            .unwrap()
            .add_children(self.get_node(node_id).unwrap().get_children());
        let dfs = self.dfs(node_id).into_iter().collect_vec();
        for node in dfs {
            // self.nodes.remove(node.get_id());
            pruned_tree.set_node(node.clone());
        }
        pruned_tree
    }
}

impl<'a> Balance<'a> for SimpleRootedTree {
    fn balance_subtree(&mut self) {
        assert!(
            self.get_cluster(self.get_root_id()).collect_vec().len() == 4,
            "Quartets have 4 leaves!"
        );
        assert!(self.is_binary(), "Cannot balance non-binary tree!");
        let root_children = self.get_node_children(self.get_root_id()).collect_vec();
        let (child1, child2) = (root_children[0].get_id(), root_children[1].get_id());
        let next_id = self.next_id();
        let split_id = self.next_id() + 1;
        match dbg!((
            (self.get_node(child1).unwrap().is_leaf()),
            (self.get_node(child2).unwrap().is_leaf())
        )) {
            (false, false) => {}
            (true, false) => {
                let mut leaf_node = self.remove_node(child1).unwrap();
                leaf_node.set_id(next_id);
                let other_leaf_id = &self
                    .get_node_children(child2)
                    .filter(|node| node.is_leaf())
                    .collect_vec()[0]
                    .get_id();
                self.split_edge((child2, *other_leaf_id), Node::new(split_id));
                self.add_child(dbg!(split_id), leaf_node);
            }
            (false, true) => {
                let mut leaf_node = self.remove_node(child2).unwrap();
                leaf_node.set_id(next_id);
                let other_leaf_id = &self
                    .get_node_children(child1)
                    .filter(|node| node.is_leaf())
                    .collect_vec()[0]
                    .get_id();
                self.split_edge((child1, *other_leaf_id), Node::new(split_id));
                self.add_child(split_id, leaf_node);
            }
            _ => {}
        }
        self.clean();
    }
}

impl<'a> CopheneticDistance<'a> for SimpleRootedTree {
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
