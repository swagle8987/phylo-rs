use crate::node::*;
use crate::tree::simple_rtree::*;
use crate::node::NodeID;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub struct PreOrdNodes
{
    stack: Vec<NodeID>,
    nodes: HashMap<NodeID, HashSet<NodeID>>
}

impl PreOrdNodes
{
    pub fn new(start_node: &NodeID, children: &HashMap<NodeID, Vec<(NodeID, Option<EdgeWeight>)>>)->Self{
        Self { stack:vec![*start_node_id], nodes: children.iter()
            .map(|(k, v)| (*k, v.iter()
                .map(|ni| ni.0).collect::<HashSet<NodeID>>()))
            .collect()}
    }
}

impl Iterator for PreOrdNodes
{
    type Item = NodeID;

    fn next(&mut self)->Option<Self::Item>{
        match self.stack.pop() {
            Some(node_id) => {
                let children_ids:HashSet<NodeID> = self.nodes.get(&node_id).cloned().expect("Invalid Node ID!");
                for child_node_id in children_ids.into_iter().sorted(){
                    self.stack.push(child_node_id)
            }
            Some(node_id)
            }
            None => None,
        }
    }
}

pub struct PostOrdNodes
{
    stack: Vec<NodeID>,
    nodes: HashMap<NodeID, HashSet<NodeID>>
}

impl PostOrdNodes
{
    pub fn new(start_node_id: &NodeID, children: &HashMap<NodeID, Vec<(NodeID, Option<EdgeWeight>)>>)->Self{
        Self { stack:vec![*start_node_id], nodes: children.iter()
                .map(|(k, v)| (*k, v.iter()
                    .map(|ni| ni.0).collect::<HashSet<NodeID>>()))
                .collect()}
    }
}

impl Iterator for PostOrdNodes
{
    type Item = NodeID;

    fn next(&mut self)->Option<Self::Item>{
        while let Some(node_id) = self.stack.pop()  {
            if self.nodes.contains_key(&node_id){
                self.stack.push(node_id);
                let children = self.nodes.remove(&node_id).unwrap();
                for child_id in children.into_iter().sorted(){
                    self.stack.push(child_id)
                }
            }
            else{
                return Some(node_id)
            }
        }
        None
    }
}
