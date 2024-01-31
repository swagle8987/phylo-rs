use crate::tree::simple_rtree::SimpleRootedTree;

pub trait InterNodeDistances
where
    Self: SimpleRootedTree + Sized
{
    fn leaf_distance_matrix(&self, weighted: bool)->impl Iterator<Item=((Self::NodeID, Self::NodeID), Self::EdgeWeight)>;
}
