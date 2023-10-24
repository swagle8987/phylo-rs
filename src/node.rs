use std::fmt::{Debug, Display};

pub type NodeID = usize;
// pub type NodeType = bool; // True for leaves, false for internal nodes

#[derive(Clone, PartialEq, Eq)]
pub enum NodeType{
    Internal(Option<String>),
    Leaf(Option<String>),
}

impl NodeType{
    pub fn new(is_leaf: bool, taxa: Option<String>)->Self{
        match is_leaf {
            true => Self::Leaf(taxa),
            false => Self::Internal(taxa),
        }
    }

    pub fn is_leaf(&self)->bool{
        match self {
            NodeType::Internal(_taxa) => false,
            NodeType::Leaf(_taxa) => true,
        }
    }

    pub fn flip(&mut self){
        match self {
            NodeType::Internal(taxa) => {*self = NodeType::Leaf(taxa.clone())},
            NodeType::Leaf(taxa) => {*self = NodeType::Internal(taxa.clone())},
        }
    }

    pub fn taxa(&self)->String{
        match self {
            NodeType::Internal(taxa) => taxa.clone().unwrap_or("".to_string()),
            NodeType::Leaf(taxa) => taxa.clone().unwrap_or("".to_string()),
        }
    }

    pub fn node_type(&self)->String{
        match self.is_leaf() {
            false => "Internal".to_string(),
            true => "Leaf".to_string(),
        }
    }
}

impl Debug for NodeType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.node_type(), self.taxa())
    }
}

impl Display for NodeType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.node_type(), self.taxa())
    }
}