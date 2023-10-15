pub mod simple_rtree;

use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::node::*;
use crate::tree::simple_rtree::*;
use crate::iter::{node_iter::*, edge_iter::*};

pub struct RootedPhyloTree{
    root: NodeID,
    nodes: HashMap<NodeID, NodeType>,
    children: HashMap<NodeID, HashSet<(Option<EdgeWeight>, NodeID)>>,
    parents: HashMap<NodeID, Option<NodeID>>,
    data: HashMap<NodeID, Box<dyn Display>>,
    leaves: HashSet<NodeID>,
}

impl RootedPhyloTree{
    pub fn new()->Self{
        RootedPhyloTree { 
            root: 0,
            nodes: HashMap::from([(0, false)]),
            children: HashMap::new(),
            parents: HashMap::from([(0, None)]),
            data: HashMap::new(),
            leaves: HashSet::new()
        }
    }

    fn leaves_of_node(&self, node_id:&NodeID, leaves:&mut Vec<NodeID>){
        if self.get_children(node_id).expect("Invalid NodeID!").is_empty(){
            leaves.push(*node_id);
        }

        for (_edge_weight, child_node_id) in self.get_children(node_id).expect("Invalid NodeID").iter(){
            self.leaves_of_node(child_node_id, leaves);
        }
    }
}

impl SimpleRTree for RootedPhyloTree{
    fn get_root(&self)->&NodeID{
        &self.root
    }

    fn get_nodes(&self)->&HashMap<NodeID, NodeType>{
        &self.nodes
    }

    fn get_children(&self, node_id: &NodeID)->Option<&HashSet<(Option<EdgeWeight>, NodeID)>>{
        self.children.get(node_id)
    }

    fn get_leaves(&self, node_id: &NodeID)->HashSet<NodeID>{
        let mut leaf_vec: Vec<NodeID> = Vec::new();
        self.leaves_of_node(node_id, &mut leaf_vec);
        leaf_vec.into_iter().collect::<HashSet<NodeID>>()
    }

    fn get_descendents(&self, node_id: &NodeID)->&Vec<NodeID>{
        todo!()
    }

    fn get_subtree(&self, node_id: &NodeID)->&Box<dyn Node>{
        todo!()
    }

    fn get_mrca(&self, node_id_list: Vec<&NodeID>)->&NodeID{
        todo!()
    }

    fn is_leaf(&self, node_id: &NodeID)->bool{
        todo!()
    }

    fn graft_subtree(&mut self, tree: Box<dyn SimpleRTree>, edge: (&NodeID, &NodeID)){
        todo!()
    }

    fn extract_subtree(&mut self, node_id: &NodeID)-> Box<dyn SimpleRTree>{
        todo!()
    }

    fn prune_subtree(&self, node_id: &NodeID)-> Box<dyn SimpleRTree>{
        todo!()
    }

    fn iter_node_pre(&self, start_node_id: &NodeID)->PreOrdNodes{
        todo!()
    }

    fn iter_node_post(&self, start_node_id: &NodeID)->PostOrdNodes{
        todo!()
    }

    fn iter_edges_pre(&self, start_node_id: &NodeID)->PreOrdEdges{
        todo!()
    }

    fn iter_edges_post(&self, start_node_id: &NodeID)->PostOrdEdges{
        todo!()
    }

    fn get_ancestors(&self, node_id: &NodeID)->Vec<&NodeID>{
        todo!()
    }

    fn leaf_distance_matrix(&self, weighted: bool)->Vec<Vec<EdgeWeight>>{
        todo!()
    }

    fn reroot_at_node(&mut self, node_id: &NodeID){
        todo!()
    }

    fn insert_internal_node(&mut self, edge: (NodeID, NodeID), edge_weights:(Option<EdgeWeight>, Option<EdgeWeight>)){
        todo!()
    }

    fn distance_from_root(&self, weighted: bool)->f64{
        todo!()
    }

    fn get_bipartition(&self, edge: (&NodeID, &NodeID)){
        todo!()
    }

}