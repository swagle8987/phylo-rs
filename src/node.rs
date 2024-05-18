pub mod simple_rnode;

use std::fmt::{Debug, Display};

use crate::node::simple_rnode::{RootedTreeNode, RootedMetaNode, RootedWeightedNode};

#[derive(Clone)]
pub struct Node{
    id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    taxa: Option<String>,
    weight: Option<f32>,
}

impl RootedTreeNode for Node
{
    type NodeID = usize;

    fn new(id: Self::NodeID)->Self{
        Node{
            id: id,
            parent: None,
            children: vec![],
            taxa: None,
            weight: None,
        }
    }

    fn get_id(&self)->Self::NodeID {
        self.id
    }

    fn set_id(&mut self, id: Self::NodeID) {
        self.id = id
    }

    fn set_parent(&mut self, parent: Option<Self::NodeID>){
        self.parent = parent;
    }
    fn get_parent(&self)->Option<Self::NodeID>{
        self.parent.clone()
    }
    fn get_children(&self)->impl ExactSizeIterator<Item = Self::NodeID> + DoubleEndedIterator
    {
        self.children.clone().into_iter()
    }
    fn add_child(&mut self, child: Self::NodeID){
        self.children.push(child);
    }
    fn remove_child(&mut self, child:&Self::NodeID) {
        self.children.retain(|x| x != child);
    }
}

impl RootedMetaNode for Node
{
    type Meta = String;
    
    fn get_taxa(&self)->Option<Self::Meta>{
        match &self.taxa{
            None => None,
            Some(t) => Some(t.to_string())
        }
    }

    fn set_taxa(&mut self, taxa: Option<Self::Meta>){
        self.taxa = match taxa{
            None => None,
            Some(t) => Some(t),
        };
    }

}

impl RootedWeightedNode for Node
{
    type Weight = f32;

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
        write!(f, "{}:{}:{}:{}", self.get_id(),self.node_type(), match self.get_taxa(){
            None => "None".to_string(),
            Some(t) => t.to_string(),
        },
        match self.get_weight() {
            None => "".to_string(),
            Some(t) => t.to_string(),

        }
    )
    }
}

impl Display for Node
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}:{}", self.get_id(),self.node_type(), match self.get_taxa(){
            None => "None".to_string(),
            Some(t) => t.to_string(),
        },
        match self.get_weight() {
            None => "".to_string(),
            Some(t) => t.to_string(),

        }
    )
    }
}

