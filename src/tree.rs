pub mod simple_rtree;

use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::node::simple_rt_node::*;
use crate::tree::simple_rtree::*;

pub struct SimpleRTree{
    root: NodeID,
    nodes: HashMap<NodeID, NodeType>,
    children: HashMap<NodeID, HashSet<(Option<EdgeWeight>, NodeID)>>,
    parents: HashMap<NodeID, NodeID>,
    data: HashMap<NodeID, Box<dyn Display>>,
    leaves: HashSet<NodeID>,
}