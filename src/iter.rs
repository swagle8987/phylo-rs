pub mod node_iter;
pub mod edge_iter;

use std::collections::VecDeque;
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use crate::{node::{simple_rnode::RootedTreeNode, Node}, tree::simple_rtree::RootedTree};

#[derive(Clone)]
pub struct BFSIterator{
    stack: VecDeque<<Node as RootedTreeNode>::NodeID>,
    nodes: HashMap<<Node as RootedTreeNode>::NodeID,Node>,
}

#[derive(Clone)]
pub struct DFSPostOrderIterator{
    stack: VecDeque<Node>,
    nodes: HashMap<<Node as RootedTreeNode>::NodeID,Node>,
}

impl BFSIterator
{
    pub fn new(tree: &impl RootedTree<NodeID = usize, Node = Node>, start_id: <Node as RootedTreeNode>::NodeID) -> BFSIterator
    {
        BFSIterator{
            stack: vec![start_id].into(),
            nodes: tree.get_nodes().into_iter().map(|x| (x.get_id(), x.clone())).collect::<HashMap<_,_>>(),
        }
    }
}

impl DFSPostOrderIterator
{
    pub fn new(tree: &impl RootedTree<NodeID = usize, Node = Node>, start_id: <Node as RootedTreeNode>::NodeID) -> DFSPostOrderIterator
    {
        let mut nodes = tree.get_nodes().map(|x| (x.get_id(), x.clone())).collect::<HashMap<_,_>>();
        let start_node = nodes.remove(&start_id).unwrap();
        DFSPostOrderIterator{
            stack: vec![start_node].into(),
            nodes: nodes,
        }
    }
}

impl Iterator for BFSIterator
{
    type Item = Node;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop_front(){
            None => return None,
            Some(curr_node_id) => {
                let curr_node = self.nodes.remove(&curr_node_id).unwrap();
                curr_node.get_children()
                    .for_each(|x| {
                        self.stack.push_back(x)
                    });
                return Some(curr_node);

            },
        };
    }
}

impl Iterator for DFSPostOrderIterator
{
    type Item = Node;
    
    fn next(&mut self) -> Option<Self::Item> {

        while let Some(node) = self.stack.pop_front(){
            let node_children = node.get_children()
                .map(|chid| self.nodes.remove(&chid))
                .filter(|chn| chn.is_some())
                .map(|x| x.unwrap())
                .collect_vec();
            if node_children.len()>0{
                self.stack.push_front(node);
                for child in node_children.into_iter().rev(){
                    self.stack.push_front(child)
                }
            }
            else{
                return Some(node);
            }
        }
        return None;
    }
}