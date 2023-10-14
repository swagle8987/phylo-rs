use std::collections::{HashMap, HashSet};

use crate::node::*;
use crate::iter::node_iter::*;
use crate::iter::edge_iter::*;

pub type EdgeWeight = f64;

pub trait SimpleRTree {
    fn get_root(&self)->NodeID;
    fn get_nodes(&self)->HashMap<NodeID, NodeType>;
    fn get_children(&self, node_id: &NodeID)->HashMap<NodeID, NodeType>;
    fn get_leaves(&self, node_id: &NodeID)->HashSet<NodeID>;
    fn get_descendents(&self, node_id: &NodeID)->Vec<NodeID>;
    fn get_subtree(&self, node_id: &NodeID)->Box<dyn Node>;
    fn get_mrca(&self, node_id_list: Vec<&NodeID>)->&NodeID;
    fn is_leaf(&self, node_id: &NodeID)->bool;
    
    fn graft_subtree(&mut self, tree: Box<dyn SimpleRTree>, edge: (&NodeID, &NodeID));
    fn extract_subtree(&mut self, node_id: &NodeID)-> Box<dyn SimpleRTree>;
    fn prune_subtree(&self, node_id: &NodeID)-> Box<dyn SimpleRTree>;

    fn iter_node_pre(&self, start_node_id: &NodeID)->PreOrdNodes;
    fn iter_node_post(&self, start_node_id: &NodeID)->PostOrdNodes;
    fn iter_edges_pre(&self, start_node_id: &NodeID)->PreOrdEdges;
    fn iter_edges_post(&self, start_node_id: &NodeID)->PostOrdEdges;
    fn get_ancestors(&self, node_id: &NodeID)->Vec<&NodeID>;

    fn phylogenetic_distance_matrix(&self)->Vec<Vec<EdgeWeight>>;
    fn reroot_at_node(&mut self, node_id: &NodeID);
    fn insert_internal_node(&mut self, edge: (NodeID, NodeID), edge_weights:(Option<EdgeWeight>, Option<EdgeWeight>));

    fn distance_from_root(&self, weighted: bool)->f64;
    fn get_bipartition(&self, edge: (&NodeID, &NodeID));
}