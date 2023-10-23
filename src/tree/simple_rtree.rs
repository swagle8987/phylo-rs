use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use crate::node::*;
use crate::iter::node_iter::*;
use crate::iter::edge_iter::*;

pub type EdgeWeight = f64;

pub trait SimpleRTree {
    /// Add node to tree
    fn add_node(&mut self)->NodeID;

    /// Sets node_id as child to parent.
    fn set_child(&mut self, node_id:&NodeID, parent_id:&NodeID, distance:Option<EdgeWeight>, taxa: String);

    /// Sets iterable of node_ids as children to parent
    fn set_children(&mut self, parent: &NodeID, children: &Vec<(NodeID, Option<EdgeWeight>)>){
        for (child_id, edge_weight) in children.iter(){
            self.set_child(child_id, &parent, edge_weight.clone(), String::new());
        }
    }

    /// Sets the edge weight between two nodes (None to unweight the edge)
    fn set_edge_weight(&mut self, parent:&NodeID, child:&NodeID, edge_weights:Option<EdgeWeight>);

    /// Returns true of node is child of parent.
    fn node_is_child_of(&self, parent:&NodeID, node:&NodeID)->bool{
        self.get_node_children(parent).iter().map(|(id, _weight)| id).contains(node)
    }

    /// Assign taxa to leaf node
    fn assign_taxa(&mut self,node:&NodeID, taxa:&str);
    
    /// Returns root node id
    fn get_root(&self)->&NodeID;
    
    /// Returns all node ids
    fn get_nodes(&self)->&HashMap<NodeID, NodeType>;

    /// Returns node degree
    fn get_node_degree(&self, node_id:&NodeID)->usize{
        self.get_node_children(node_id).len() + match self.get_node_parent(node_id) {
            Some(_) => 1,
            None => 0
        }
    }

    /// Check if tree is weighted
    fn is_weighted(&self)->bool{
        for (_, _, edge_weight) in self.iter_edges_post(self.get_root()){
            if edge_weight!=None{
                return true;
            }
        }
        false
    }
    
    /// Returns children node ids for given node id 
    fn get_node_children(&self, node_id: &NodeID)->&Vec<(NodeID, Option<EdgeWeight>)>;

    /// Returns node parent
    fn get_node_parent(&self, node_id:&NodeID)->Option<&NodeID>;
    
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
    fn get_ancestors_pre(&self, node_id: &NodeID)->Vec<NodeID>;

    /// Returns pairwise distance matrix of the taxa. If weighted is true, then returns sum of edge weights along paths connecting leaves of tree
    fn leaf_distance_matrix(&self, weighted: bool)->Vec<Vec<EdgeWeight>>;

    /// Rerootes tree at given node.
    fn reroot_at_node(&mut self, node_id: &NodeID);
    
    /// Rerootes tree at edge.
    fn reroot_at_edge(&mut self, edge: (&NodeID, &NodeID));

    /// Inserts node in the middle of edge given by pair of node ids
    fn split_edge(&mut self, edge: (NodeID, NodeID), edge_weights:(Option<EdgeWeight>, Option<EdgeWeight>));

    /// Returns distance of node from root. If weighted is true, it returns sum of edges from root to self.
    fn distance_from_root(&self, node: &NodeID, weighted: bool)->f64;

    /// Returns bipartition induced by edge
    fn get_bipartition(&self, edge: (&NodeID, &NodeID))->(HashSet<NodeID>, HashSet<NodeID>);

    /// Returns cluster of node
    fn get_cluster(&self, node_id: &NodeID)-> HashSet<NodeID>;

    /// Cleans self by removing 1) internal nodes (other than root) with degree 2, 2) Floating root nodes, 3) self loops
    fn clean(&mut self);

    /// Get node taxa
    fn get_taxa(&self, node_id:&NodeID)->&String;

    /// Get edge weight
    fn get_edge_weight(&self, parent_id: &NodeID, child_id:&NodeID)->Option<&EdgeWeight>{
        for node_id in self.get_node_children(parent_id).iter(){
            if node_id.0==*child_id{
                return node_id.1.as_ref();
            }
        }
        return None;
    }

    /// return subtree as newick string
    fn subtree_to_newick(&self, node_id:&NodeID, edge_weight:Option<EdgeWeight>)->String{
        match self.is_leaf(node_id){
            true => {
                match edge_weight {
                    Some(w) => {format!("{}:{}", self.get_taxa(node_id), w)},
                    _ => {format!("{}", self.get_taxa(node_id))}
                }
            }
            false => {
                let mut tmp = String::new();
                tmp.push('(');
                for (child_id, w) in self.get_node_children(node_id){
                    let child_str = format!("{},", self.subtree_to_newick(child_id, *w));
                    tmp.push_str(&child_str);
                }
                tmp.pop();
                tmp.push(')');
                return tmp;
            }
        }
    }

    /// writes full tree in newick format
    fn to_newick(&self)->String{
        format!("{};", self.subtree_to_newick(self.get_root(), None))
    }
}
