/// Module with traits for tree traversals
pub mod node_iter;

use crate::{node::{Node, NodeID}, prelude::*};
use itertools::Itertools;
use std::collections::VecDeque;
use vers_vecs::BinaryRmq;

/// Struct for BFS iteration of nodes in a tree
#[derive(Clone)]
pub struct BFSIterator<'a,T,W,Z> 
where 
    T: NodeTaxa,
    W: EdgeWeight,
    Z: NodeWeight,
{
    stack: VecDeque<usize>,
    nodes: Vec<Option<&'a Node<T,W,Z>>>,
}

/// Struct for DFS postfix iteration of nodes in a tree
#[derive(Clone)]
pub struct DFSPostOrderIterator<'a,T,W,Z> 
where 
    T: NodeTaxa,
    W: EdgeWeight,
    Z: NodeWeight,
{
    stack: VecDeque<&'a Node<T,W,Z>>,
    nodes: Vec<Option<&'a Node<T,W,Z>>>,
}

impl<'a,T,W,Z> BFSIterator<'a,T,W,Z> 
where 
    T: NodeTaxa,
    W: EdgeWeight,
    Z: NodeWeight,
{
    /// Creates a new BFS iterator of a tree
    pub fn new(tree: &'a impl RootedTree<Node = Node<T,W,Z>>, start_id: usize) -> BFSIterator<'a,T,W,Z> {
        let max_id = tree.get_node_ids().max().unwrap();
        let mut nodes = vec![None; max_id + 1];
        tree.get_nodes()
            .for_each(|node| nodes[node.get_id()] = Some(node));
        BFSIterator {
            stack: vec![start_id].into(),
            nodes,
        }
    }
}

impl<'a,T,W,Z> DFSPostOrderIterator<'a,T,W,Z> 
where 
    T: NodeTaxa,
    W: EdgeWeight,
    Z: NodeWeight,
{
    /// Creates a new DFS postfix iterator of a tree
    pub fn new(
        tree: &'a impl RootedTree<Node = Node<T,W,Z>>,
        start_id: usize,
    ) -> DFSPostOrderIterator<'a,T,W,Z> {
        let max_id = tree.get_node_ids().max().unwrap();
        let mut nodes = vec![None; max_id + 1];
        tree.get_nodes()
            .for_each(|node| nodes[node.get_id()] = Some(node));
        let start_node = nodes[start_id].take().unwrap();
        DFSPostOrderIterator {
            stack: vec![start_node].into(),
            nodes,
        }
    }
}

impl<'a,T,W,Z> Iterator for BFSIterator<'a,T,W,Z> 
where 
    T: NodeTaxa,
    W: EdgeWeight,
    Z: NodeWeight,
{
    type Item = &'a Node<T,W,Z>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop_front() {
            None => None,
            Some(curr_node_id) => {
                let curr_node = self.nodes[curr_node_id].take().unwrap();
                curr_node
                    .get_children()
                    .for_each(|x| self.stack.push_back(x));
                Some(curr_node)
            }
        }
    }
}

impl<'a,T,W,Z> Iterator for DFSPostOrderIterator<'a,T,W,Z> 
where 
    T: NodeTaxa,
    W: EdgeWeight,
    Z: NodeWeight,
{
    type Item = &'a Node<T,W,Z>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop_front() {
            let node_children = node
                .get_children()
                .filter_map(|chid| self.nodes[chid].take())
                .collect_vec();
            if !node_children.is_empty() {
                self.stack.push_front(node);
                for child in node_children.into_iter().rev() {
                    self.stack.push_front(child)
                }
            } else {
                return Some(node);
            }
        }
        None
    }
}

pub struct LCA{
    /// Field to hold precomputed euler tour for constant-time LCA queries
    euler_traversal: Vec<NodeID>,
    /// Field to hold precomputed first-appearance for constant-time LCA queries
    first_appearance_index: Vec<Option<usize>>,
    /// Field to hold precomputed range-minimum-query for constant-time LCA queries
    range_min_query: BinaryRmq,
}

impl LCA{
    /// Creates new empty LCA map
    pub fn new<T: NodeTaxa,W: EdgeWeight,Z: NodeWeight>(tree: &SimpleRootedTree<T,W,Z>)->Self{
        // compute first-appearance index
        let max_id = tree.get_node_ids().max().unwrap();
        let mut index = vec![None; max_id + 1];
        match tree.get_precomputed_walk() {
            Some(walk) => {
                for node_id in tree.get_node_ids() {
                    index[node_id] = Some(walk.iter().position(|x| x == &node_id).unwrap());
                }
            }
            None => {
                let walk = tree.euler_walk_ids(tree.get_root_id()).collect_vec();
                for node_id in tree.get_node_ids() {
                    index[node_id] = Some(walk.iter().position(|x| x == &node_id).unwrap());
                }
            }
        }
        let mut fai_vec = vec![None; max_id + 1];
        for id in tree.get_node_ids() {
            fai_vec[id] = index[id];
        }

        LCA { euler_traversal: vec![], first_appearance_index: fai_vec, range_min_query: BinaryRmq::from_vec(vec![]) }
    }

    pub fn get_lca_id(&self, node_id_vec: &[NodeID]) -> NodeID {
        if node_id_vec.len() == 1 {
            return node_id_vec[0];
        }
        let min_pos = node_id_vec
            .iter()
            .map(|x| self.first_appearance_index[*x].unwrap())
            .min()
            .unwrap();
        let max_pos = node_id_vec
            .iter()
            .map(|x| self.first_appearance_index[*x].unwrap())
            .max()
            .unwrap();

        self.euler_traversal[self.range_min_query.range_min(min_pos, max_pos)]
    }
}
