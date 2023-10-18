use std::collections::{HashMap, HashSet};

use crate::node::*;
use crate::iter::node_iter::*;
use crate::iter::edge_iter::*;

pub type EdgeWeight = f64;

pub trait SimpleRTree {
    /// Add node to tree
    fn add_node(&mut self, children: Vec<(NodeID, Option<EdgeWeight>)>, parent:Option<NodeID>, leaf_id:Option<String>, parent_edge_weight: Option<EdgeWeight>)->NodeID;

    /// Add child to node
    fn add_child(&mut self,parent:&NodeID, child:&NodeID, distance:Option<EdgeWeight>);

    /// Assign taxa to leaf node
    fn assign_taxa(&mut self,node:&NodeID, taxa:&str);
    
    /// Returns root node id
    fn get_root(&self)->&NodeID;
    
    /// Returns all node ids
    fn get_nodes(&self)->&HashMap<NodeID, NodeType>;
    
    /// Returns children node ids for given node id 
    fn get_children(&self, node_id: &NodeID)->Option<&Vec<(NodeID, Option<EdgeWeight>)>>;
    
    /// Returns all leaf node ids
    fn get_leaves(&self, node_id: &NodeID)->HashSet<NodeID>;
    
    /// Returns full subtree rooted at given node
    fn get_subtree(&self, node_id: &NodeID)->Box<dyn SimpleRTree>;
    
    /// Returns most recent common ancestor of give node set
    fn get_mrca(&self, node_id_list: Vec<&NodeID>)->NodeID;
    
    /// Checks if the given node is a leaf node
    fn is_leaf(&self, node_id: &NodeID)->bool;
    
    /// Attaches input tree to self by spliting an edge
    fn graft_subtree(&mut self, tree: Box<dyn SimpleRTree>, edge: (&NodeID, &NodeID));
    
    /// Returns subtree starting at given node, while corresponding nodes from self.
    fn extract_subtree(&mut self, node_id: &NodeID)-> Box<dyn SimpleRTree>;

    ///Returns an iterator that iterates over the nodes in Pre-order
    fn iter_node_pre(&self, start_node_id: &NodeID)->PreOrdNodes;
    
    ///Returns an iterator that iterates over the nodes in Post-order
    fn iter_node_post(&self, start_node_id: &NodeID)->PostOrdNodes;
    
    ///Returns an iterator that iterates over the edges in Pre-order
    fn iter_edges_pre(&self, start_node_id: &NodeID)->PreOrdEdges;
    
    ///Returns an iterator that iterates over the edges in Post-order
    fn iter_edges_post(&self, start_node_id: &NodeID)->PostOrdEdges;

    /// Returns all node ids in path from root to given node
    fn get_ancestors(&self, node_id: &NodeID)->Vec<&NodeID>;

    /// Returns pairwise distance matrix of the taxa. If weighted is true, then returns sum of edge weights along paths connecting leaves of tree
    fn leaf_distance_matrix(&self, weighted: bool)->Vec<Vec<EdgeWeight>>;

    /// Rerootes tree at given node.
    fn reroot_at_node(&mut self, node_id: &NodeID);
    
    /// Rerootes tree at edge.
    fn reroot_at_edge(&mut self, edge: (&NodeID, &NodeID));

    /// Inserts node in the middle of edge given by pair of node ids
    fn insert_internal_node(&mut self, edge: (NodeID, NodeID), edge_weights:(Option<EdgeWeight>, Option<EdgeWeight>));

    /// Returns distance of node from root. If weighted is true, it returns sum of edges from root to self.
    fn distance_from_root(&self, weighted: bool)->f64;

    /// Returns bipartition induced by edge
    fn get_bipartition(&self, edge: (&NodeID, &NodeID))->(HashSet<NodeID>, HashSet<NodeID>);

    /// Returns cluster of node
    fn get_cluster(&self, node_id: &NodeID)-> HashSet<NodeID>;
}
