use crate::tree::simple_rtree::RootedTree;

pub trait Clusters
where
    Self: RootedTree + Sized
{
    fn get_cluster(&self, node_id: Self::NodeID)-> impl IntoIterator<Item=Self::NodeID, IntoIter = impl ExactSizeIterator<Item = Self::NodeID>>;
    fn get_bipartition(&self, edge: (Self::NodeID, Self::NodeID))->(impl IntoIterator<Item=Self::NodeID>, impl IntoIterator<Item=Self::NodeID, IntoIter = impl ExactSizeIterator<Item = Self::NodeID>>);
}