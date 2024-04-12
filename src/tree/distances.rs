// use crate::tree::{WeightedTree, RootedTree};

// pub trait InterNodeDistances
// where
//     Self: WeightedTree + Sized
// {
//     fn leaf_distance_matrix(&self, weighted: bool)->impl IntoIterator<Item=((Self::NodeID, Self::NodeID), Self::EdgeWeight)>;
//     fn node_distance_matrix(&self, weighted: bool)->impl IntoIterator<Item=((Self::NodeID, Self::NodeID), Self::EdgeWeight)>;
//     fn distance_from_root(&self, node_id: Self::NodeID, weighted: bool)->Self::EdgeWeight;
//     fn distance_from_node(&self, node1: Self::NodeID, node2: Self::NodeID, weighted: bool)->Self::EdgeWeight;
// }

// pub trait PathFunction<T>
// where
//     Self: RootedTree + Sized,
//     T: PartialOrd + PartialEq
// {
//     fn zeta(&self, node: <Self as RootedTree>::NodeID)->T;
// }
