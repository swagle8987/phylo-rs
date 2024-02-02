use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone)]
pub enum NodeType
{
    Internal,
    Leaf,
}

pub trait RootedTreeNode
{
    type Taxa: Display + Debug + Eq + PartialEq + Clone + Ord;
    type NodeID: Display + Debug + Hash + Clone + Drop + Ord;
    
    fn is_leaf(&self)->bool;
    fn flip(&mut self);
    fn get_taxa(&self)->Option<Self::Taxa>;
    fn set_taxa(&mut self, taxa: Option<Self::Taxa>);
    fn get_id(&self)->Self::NodeID;
    fn set_id(&mut self, id: Self::NodeID);
    fn get_parent(&self)->Option<Self::NodeID>;
    fn set_parent(&mut self, parent: Option<Self::NodeID>);
    fn get_children(&self)->impl IntoIterator<Item=Self::NodeID>;
    fn add_child(&mut self, child:Self::NodeID);
    fn remove_child(&mut self, child:Self::NodeID);

    fn node_type(&self)->String{
        match self.is_leaf() {
            false => "Internal".to_string(),
            true => "Leaf".to_string(),
        }
    }
    fn add_children(&mut self, children: impl IntoIterator<Item=Self::NodeID>){
        for child in children.into_iter(){
            self.add_child(child);
        }
    }
    fn remove_children(&mut self, children: impl IntoIterator<Item=Self::NodeID>){
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
    fn num_children(&self)->usize
    {
        self.get_children().into_iter().collect::<Vec<Self::NodeID>>().len()
    }
    fn degree(&self)->usize
    {
        match self.get_parent()
        {
            Some(_) => self.num_children()+1,
            None => self.num_children()
        }
    }
}

pub trait WeightedNode
where
    Self: RootedTreeNode
{
    type Weight: Display + Debug + Clone + Add<Output=Self::Weight> + AddAssign + Sub<Output=Self::Weight> + SubAssign;

    fn get_weight(&self)->Option<Self::Weight>;
    fn set_weight(&mut self, w: Option<Self::Weight>);
    fn unweight(&mut self){
        self.set_weight(None);
    }
}
