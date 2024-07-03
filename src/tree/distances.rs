use crate::node::simple_rnode::{RootedTreeNode, RootedZetaNode};
use crate::tree::RootedTree;

// pub trait InterNodeDistances
// where
//     Self: WeightedTree + Sized
// {
//     fn leaf_distance_matrix(&self, weighted: bool)->impl IntoIterator<Item=((Self::NodeID, Self::NodeID), Self::EdgeWeight)>;
//     fn node_distance_matrix(&self, weighted: bool)->impl IntoIterator<Item=((Self::NodeID, Self::NodeID), Self::EdgeWeight)>;
//     fn distance_from_root(&self, node_id: Self::NodeID, weighted: bool)->Self::EdgeWeight;
//     fn distance_from_node(&self, node1: Self::NodeID, node2: Self::NodeID, weighted: bool)->Self::EdgeWeight;
// }

pub trait PathFunction<'a>: RootedTree<'a>
where
    <Self as RootedTree<'a>>::Node: RootedZetaNode,
{
    fn set_zeta(
        &'a mut self,
        zeta_func: fn(
            &Self,
            <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
        ) -> <<Self as RootedTree<'a>>::Node as RootedZetaNode>::Zeta,
    );
    fn get_zeta(
        &'a self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> Option<<<Self as RootedTree<'a>>::Node as RootedZetaNode>::Zeta>;

    fn is_zeta_set(
        &self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> bool;
    // {
    //     self.get_node(node_id).unwrap().is_zeta_set()
    // }

    fn is_all_zeta_set(&'a self) -> bool {
        !self.get_nodes().any(|x| !x.is_zeta_set())
    }

    fn set_node_zeta(
        &'a mut self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
        zeta: Option<<<Self as RootedTree<'a>>::Node as RootedZetaNode>::Zeta>,
    );
}
