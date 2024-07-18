use crate::node::simple_rnode::RootedZetaNode;
use crate::tree::{RootedTree, TreeNodeID};

pub type TreeNodeZeta<'a, T> = <<T as RootedTree<'a>>::Node as RootedZetaNode>::Zeta;


pub trait PathFunction<'a>: RootedTree<'a>
where
    <Self as RootedTree<'a>>::Node: RootedZetaNode,
{
    fn set_zeta(&'a mut self, zeta_func: fn(&Self, TreeNodeID<'a, Self>) -> TreeNodeZeta<'a, Self>);
    
    fn get_zeta(&self, node_id: TreeNodeID<'a, Self>) -> Option<TreeNodeZeta<'a, Self>>;

    fn is_zeta_set(&self, node_id: TreeNodeID<'a, Self>) -> bool;

    fn is_all_zeta_set(&self) -> bool;

    fn set_node_zeta(
        &mut self,
        node_id: TreeNodeID<'a, Self>,
        zeta: Option<TreeNodeZeta<'a, Self>>,
    );
}
