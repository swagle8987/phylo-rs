use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub trait UnrootedTreeNode{
    type Weight: Display + Debug + Clone + Add<Output=Self::Weight> + AddAssign + Sub<Output=Self::Weight> + SubAssign;
    type Taxa: Display + Debug + Eq + PartialEq + Clone + Ord;
    type NodeID: Display + Debug + Hash + Copy + Clone + Ord + Add<Output = Self::NodeID> + AddAssign + Sub<Output = Self::NodeID> + SubAssign;
    
    fn new(id: Self::NodeID, is_leaf: bool)->Self;
    fn is_leaf(&self)->bool;
    fn flip(&mut self);
    fn get_taxa(&self)->Option<Self::Taxa>;
    fn set_taxa(&mut self, taxa: Option<Self::Taxa>);
    fn get_id(&self)->Self::NodeID;
    fn set_id(&mut self, id: Self::NodeID);
    fn get_neighbours(&self)->&Vec<(Self::NodeID, Weight)>;
    fn add_neighbour(&mut self, child:(Self::NodeID, Weight));
    fn remove_neighbour(&mut self, child:Self::NodeID);
    fn node_type(&self)->String{
        match self.is_leaf() {
            false => "Internal".to_string(),
            true => "Leaf".to_string(),
        }
    }
    fn add_neighbours(&mut self, neighbours: Vec<(Self::NodeID, Weight)>){
        for neighbour in neighbours.into_iter(){
            self.add_neighbour(neighbour);
        }
    }
    fn remove_neighbours(&mut self, neighbours: Vec<Self::NodeID>){
        for neighbour in neighbours.into_iter(){
            self.remove_neighbour(neighbours);
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