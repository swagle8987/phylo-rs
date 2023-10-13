use std::ops::Index;

pub type NodeID = usize;
pub type NodeType = bool; // True for leaves, false for internal nodes

pub trait SimpleRTNode {
    fn is_leaf(&self)->bool;
}

impl SimpleRTNode for NodeType{
    fn is_leaf(&self)->bool {
        *self
    }
}