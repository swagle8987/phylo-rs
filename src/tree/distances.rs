use crate::node::simple_rnode::RootedZetaNode;
use crate::tree::{RootedTree, TreeNodeID};

pub type TreeNodeZeta<'a, T> = <<T as RootedTree<'a>>::Node as RootedZetaNode>::Zeta;


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
            TreeNodeID<'a, Self>,
        ) -> TreeNodeZeta<'a, Self>,
    );
    fn get_zeta(
        &'a self,
        node_id: TreeNodeID<'a, Self>,
    ) -> Option<TreeNodeZeta<'a, Self>>;

    fn is_zeta_set(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> bool;
    // {
    //     self.get_node(node_id).unwrap().is_zeta_set()
    // }

    fn is_all_zeta_set(&'a self) -> bool {
        !self.get_nodes().any(|x| !x.is_zeta_set())
    }

    fn set_node_zeta(
        &'a mut self,
        node_id: TreeNodeID<'a, Self>,
        zeta: Option<TreeNodeZeta<'a, Self>>,
    );
}
