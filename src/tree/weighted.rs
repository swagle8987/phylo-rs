use super::RootedTree;

pub trait WeightedTree
where
    Self: RootedTree + Sized
{
    fn unweight(self)->impl RootedTree;
    fn set_edge_weight(&mut self, edge:(Self::NodeID, Self::NodeID), edge_weight:Option<Self::EdgeWeight>);
    fn is_weighted(&self)->bool;
    fn get_edge_weight(&self, parent_id: Self::NodeID, child_id:Self::NodeID)->Option<Self::EdgeWeight>;
}