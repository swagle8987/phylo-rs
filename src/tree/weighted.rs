use std::{fmt::{Debug, Display}, ops::{Add, AddAssign, Sub, SubAssign}};

use super::RootedTree;

pub trait WeightedTree
where
    Self: RootedTree + Sized
{
    type EdgeWeight: Display + Debug + Clone + Add<Output = Self::EdgeWeight> + AddAssign + Sub<Output = Self::EdgeWeight> + SubAssign;

    fn unweight(self);
    fn set_edge_weight(&mut self, edge:(Self::NodeID, Self::NodeID), edge_weight:Option<Self::EdgeWeight>);
    fn is_weighted(&self)->bool;
    fn get_edge_weight(&self, parent_id: Self::NodeID, child_id:Self::NodeID)->Option<Self::EdgeWeight>;
}