pub type NodeID = usize;
pub type NodeType = bool; // True for leaves, false for internal nodes

pub trait Node {
    fn is_leaf(&self)->bool;
}

impl Node for NodeType{
    fn is_leaf(&self)->bool {
        *self
    }
}