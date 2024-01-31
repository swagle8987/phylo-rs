use super::weighted::WeightedTree;

pub trait InterNodeDistances
where
    Self: WeightedTree + Sized
{
    fn leaf_distance_matrix(&self, weighted: bool)->impl IntoIterator<Item=((Self::NodeID, Self::NodeID), Self::EdgeWeight)>;
    fn node_distance_matrix(&self, weighted: bool)->impl IntoIterator<Item=((Self::NodeID, Self::NodeID), Self::EdgeWeight)>;
    fn distance_from_root(&self, node_id: Self::NodeID, weighted: bool)->Self::EdgeWeight;
    fn distance_from_node(&self, node1: Self::NodeID, node2: Self::NodeID, weighted: bool)->Self::EdgeWeight;
}
