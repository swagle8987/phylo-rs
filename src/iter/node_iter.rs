use crate::node::*;
use crate::tree::simple_rtree::*;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;


pub struct PreOrdNodes
{
    stack: Vec<NodeID>,
    nodes: HashMap<NodeID, HashSet<NodeID>>
}

impl PreOrdNodes
{
    pub fn new(start_node_id: &NodeID, _tree: &HashMap<NodeID, (EdgeWeight, NodeID)>)->Self{
        Self { stack:vec![*start_node_id], nodes: HashMap::new()}
    }
}

impl Iterator for PreOrdNodes
{
    type Item = NodeID;

    fn next(&mut self)->Option<Self::Item>{
        todo!()
    }
}

pub struct PostOrdNodes
{
    stack: Vec<NodeID>,
    nodes: HashMap<NodeID, HashSet<NodeID>>
}

impl PostOrdNodes
{
    pub fn new(start_node_id: &NodeID, _tree: &HashMap<NodeID, (EdgeWeight, NodeID)>)->Self{
        Self { stack:vec![*start_node_id], nodes: HashMap::new()}
    }
}

impl Iterator for PostOrdNodes
{
    type Item = NodeID;

    fn next(&mut self)->Option<Self::Item>{
        todo!()
    }
}