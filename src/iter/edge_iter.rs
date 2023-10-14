use crate::node::*;
use crate::tree::simple_rtree::*;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;


pub struct PreOrdEdges
{
    stack: Vec<(NodeID, NodeID)>,
    nodes: HashMap<NodeID, HashSet<NodeID>>
}

impl PreOrdEdges
{
    pub fn new(_tree: &HashMap<NodeID, (EdgeWeight, NodeID)>)->Self{
        Self { stack:vec![], nodes: HashMap::new()}
    }
}

impl Iterator for PreOrdEdges
{
    type Item = (NodeID, NodeID);

    fn next(&mut self)->Option<Self::Item>{
        todo!();
    }
}

pub struct PostOrdEdges
{
    stack: Vec<(NodeID, NodeID)>,
    nodes: HashMap<NodeID, HashSet<NodeID>>
}

impl PostOrdEdges
{
    pub fn new(_tree: &HashMap<NodeID, (EdgeWeight, NodeID)>)->Self{
        Self { stack:vec![], nodes: HashMap::new()}
    }
}

impl Iterator for PostOrdEdges
{
    type Item = (NodeID, NodeID);

    fn next(&mut self)->Option<Self::Item>{
        todo!();
    }
}