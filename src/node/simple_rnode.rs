use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::rc::Rc;

#[derive(Clone)]
pub enum NodeType
{
    Internal,
    Leaf,
}

pub trait RootedTreeNode
{
    type Weight: Display + Debug + Clone + Add<Output=Self::Weight> + AddAssign + Sub<Output=Self::Weight> + SubAssign;
    type Taxa: Display + Debug + Eq + PartialEq + Clone + Ord;
    type NodeID: Display + Debug + Hash + Copy + Clone + Ord + Add<Output = Self::NodeID> + AddAssign + Sub<Output = Self::NodeID> + SubAssign;
    
    fn new(id: Self::NodeID, is_leaf: bool)->Self;
    fn is_leaf(&self)->bool;
    fn flip(&mut self);
    fn get_taxa(&self)->Option<Rc<Self::Taxa>>;
    fn set_taxa(&mut self, taxa: Option<Self::Taxa>);
    fn get_weight(&self)->Option<Self::Weight>;
    fn set_weight(&mut self, w: Option<Self::Weight>);
    fn get_id(&self)->Rc<Self::NodeID>;
    fn set_id(&mut self, id: Self::NodeID);
    fn get_parent(&self)->Option<&Rc<Self::NodeID>>;
    fn set_parent(&mut self, parent: Option<Rc<Self::NodeID>>);
    fn get_children(&self)->impl Iterator<Self::NodeID>;
    fn add_child(&mut self, child:Rc<Self::NodeID>);
    fn remove_child(&mut self, child:Rc<Self::NodeID>);
    fn unweight(&mut self){
        self.set_weight(None);
    }
    fn node_type(&self)->String{
        match self.is_leaf() {
            false => "Internal".to_string(),
            true => "Leaf".to_string(),
        }
    }
    fn add_children(&mut self, children: Vec<Rc<Self::NodeID>>){
        for child in children.into_iter(){
            self.add_child(child);
        }
    }
    fn remove_children(&mut self, children: Vec<Rc<Self::NodeID>>){
        for child in children.into_iter(){
            self.remove_child(child);
        }
    }
    fn to_leaf(&mut self){
        match self.is_leaf(){
            true => {},
            false => {self.flip()}
        }
    }
    fn to_internal(&mut self){
        match self.is_leaf(){
            true => {self.flip()},
            false => {}
        }
    }
}