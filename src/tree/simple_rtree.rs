use std::collections::{HashMap, HashSet};

use crate::node::simple_rt_node::*;

pub type EdgeWeight = f64;

pub trait SimpleRTree {
    fn get_root(&self)->NodeID;
    fn get_nodes(&self)->HashMap<NodeID, NodeType>;
    fn get_children(&self, node_id: &NodeID)->HashMap<NodeID, NodeType>;
    fn get_leaves(&self, node_id: &NodeID)->HashSet<NodeID>;
}