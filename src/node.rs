pub mod simple_rnode;

use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::node::simple_rnode::{RootedTreeNode, NodeType};

pub type NodeID = usize;

pub struct Node{
    id: Rc<NodeID>,
    parent: Option<Rc<NodeID>>,
    children: Vec<Rc<NodeID>>,
    taxa: Option<Rc<String>>,
    weight: Option<f64>,
    node_type: NodeType,
}

impl RootedTreeNode for Node
{
    type NodeID = NodeID;
    type Taxa = String;
    type Weight = f64;

    fn new(id: NodeID, is_leaf: bool)->Self{
        Node{
            id: Rc::new(id),
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

    fn get_taxa(&self)->Option<Rc<Self::Taxa>>{
        match self.taxa{
            None => None,
            Some(t) => Some(Rc::clone(&t))
        }
    }

    fn get_weight(&self)->Option<Self::Weight>{
        self.weight.clone()
    }

    fn set_weight(&mut self, w: Option<Self::Weight>){
        self.weight = w;
    }

    fn get_id(&self)->Rc<Self::NodeID> {
        Rc::clone(&self.id)
    }

    fn set_id(&mut self, id: Self::NodeID) {
        self.id = Rc::new(id)
    }

    fn set_taxa(&mut self, taxa: Option<String>){
        self.taxa = match taxa{
            None => None,
            Some(t) => Some(Rc::new(t)),
        };
    }
    fn set_parent(&mut self, parent: Option<Rc<NodeID>>){
        self.parent = parent;
    }
    fn get_parent(&self)->Option<&Rc<NodeID>>{
        self.parent.as_ref()
    }
    fn get_children(&self)->&Vec<Rc<NodeID>>{
        &self.children
    }
    fn add_child(&mut self, child: Rc<NodeID>){
        self.children.push(child);
    }
    fn remove_child(&mut self, child:Rc<Self::NodeID>) {
        self.children.retain(|x| x != &child);
    }
    fn remove_children(&mut self, children: Vec<Rc<Self::NodeID>>){
        self.children.retain(|x| children.contains(&x));
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