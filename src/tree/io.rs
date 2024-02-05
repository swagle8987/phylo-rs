use std::fmt::Display;

use super::simple_rtree::RootedTree;

pub trait Newick
where
    Self: RootedTree + Sized
{
    fn from_newick(newick_str: &[u8])->Self;
    fn subtree_to_newick(&self, node_id: Self::NodeID)-> impl Display;
    fn to_newick(&self)->impl Display
    {
        self.subtree_to_newick(self.get_root_id())
    }
}

pub trait Nexus
{
    fn from_nexus(newick_str: &[u8])->impl RootedTree;
    fn to_nexus(&self)->impl Display;
}