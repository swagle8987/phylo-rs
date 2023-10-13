use std::ops::Index;

use crate::node::simple_rt_node::{NodeID, SimpleRTNode};
use crate::taxa::*;

pub trait SimpleRTree {
    fn get_root_id(&self)->&NodeID;
    fn get_root(&self)->Box<&dyn SimpleRTNode>;


    fn get_nodes(&self)->Box<dyn Index<usize, Output = NodeID>>;
    fn get_node(&self, node_id: &NodeID)->Box<&dyn SimpleRTNode>;
    fn get_node_mut(&mut self, node_id: &NodeID)->Box<&mut dyn SimpleRTNode>;
    fn add_node(&mut self, node_id: NodeID, node: dyn SimpleRTNode);

    fn get_leaf_ids(&self)->Vec<&NodeID>;
}

pub trait RPhyloTree<T>: SimpleRTree {
    fn get_taxa_ids(&self)->Vec<TaxaID>;
    fn get_taxa(&self)->Vec<Box<&dyn Taxa<T>>>;
}