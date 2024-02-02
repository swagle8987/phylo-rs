pub mod simple_rnode;

use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::node::simple_rnode::{RootedTreeNode, NodeType};

use self::simple_rnode::WeightedNode;

pub type NodeID = Rc<usize>;

#[derive(Clone)]
pub struct Node{
    id: NodeID,
    parent: Option<NodeID>,
    children: Vec<NodeID>,
    taxa: Option<Rc<String>>,
    weight: Option<f64>,
    node_type: NodeType,
}

impl RootedTreeNode for Node
{
    type NodeID = NodeID;
    type Taxa = String;

    fn new(id: NodeID, is_leaf: bool)->Self{
        Node{
            id: Rc::clone(&id),
            parent: None,
            children: vec![],
            taxa: None,
            weight: None,
            node_type: match is_leaf {
                true => NodeType::Leaf,
                false => NodeType::Internal,
            },
        }
    }

    fn is_leaf(&self)->bool{
        match self.node_type{
            NodeType::Internal => false,
            NodeType::Leaf => true,
        }
    }

    fn flip(&mut self){
        self.node_type= match self.node_type{
            NodeType::Internal => NodeType::Leaf,
            NodeType::Leaf => NodeType::Internal,
        };
    }

    fn get_taxa(&self)->Option<Self::Taxa>{
        match &self.taxa{
            None => None,
            Some(t) => Some(t.to_string())
        }
    }

    fn get_id(&self)->Self::NodeID {
        Rc::clone(&self.id)
    }

    fn set_id(&mut self, id: Self::NodeID) {
        self.id = Rc::clone(&id)
    }

    fn set_taxa(&mut self, taxa: Option<String>){
        self.taxa = match taxa{
            None => None,
            Some(t) => Some(Rc::new(t)),
        };
    }
    fn set_parent(&mut self, parent: Option<Self::NodeID>){
        self.parent = parent;
    }
    fn get_parent(&self)->Option<Self::NodeID>{
        self.parent.clone()
    }
    fn get_children(&self)->impl IntoIterator<Item=Self::NodeID, IntoIter = impl ExactSizeIterator<Item = Self::NodeID>>{
        self.children.clone()
    }
    fn add_child(&mut self, child: Self::NodeID){
        self.children.push(child);
    }
    fn remove_child(&mut self, child:Self::NodeID) {
        self.children.retain(|x| x != &child);
    }
}

impl WeightedNode for Node
{
    type Weight = f64;

    fn get_weight(&self)->Option<Self::Weight>{
        self.weight.clone()
    }

    fn set_weight(&mut self, w: Option<Self::Weight>){
        self.weight = w;
    }

}

impl Debug for Node
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.node_type(), match self.get_taxa(){
            None => "None".to_string(),
            Some(t) => t.to_string(),
        })
    }
}

impl Display for Node
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.node_type(), match self.get_taxa(){
            None => "None".to_string(),
            Some(t) => t.to_string(),
        })
    }
}
