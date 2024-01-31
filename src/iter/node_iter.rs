use crate::tree::simple_rtree::SimpleRootedTree;

pub trait Clusters
where
    Self: SimpleRootedTree + Sized
{
    fn get_cluster(&self, node_id: Self::NodeID)-> impl IntoIterator<Item=Self::NodeID>;
    fn get_bipartition(&self, edge: (Self::NodeID, Self::NodeID))->(impl IntoIterator<Item=Self::NodeID>, impl IntoIterator<Item=Self::NodeID>);
}