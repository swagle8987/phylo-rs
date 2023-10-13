use std::ops::Index;

pub type NodeID = usize;

pub trait SimpleRTNode {
    fn set_parent_id(&mut self, parent: NodeID);
    fn get_parent_id(&self)->Option<&NodeID>;
    fn get_edge_weight(&self)-> Option<&f64>;
    fn set_edge_weight(&mut self, edge_weight:f64);
    fn add_child(&mut self, child_id: NodeID);
    fn has_children(&self)->bool;
    fn get_children(&self)->Box<dyn Index<usize, Output=NodeID>>;
    fn is_leaf(&self)->bool;
}