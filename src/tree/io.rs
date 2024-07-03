use num::{Num, NumCast};
use std::fmt::{Debug, Display};

use crate::node::simple_rnode::{RootedMetaNode, RootedWeightedNode, RootedTreeNode};
use crate::tree::RootedTree;

pub trait Newick<'a>: RootedTree<'a>
where
    <Self as RootedTree<'a>>::Node:
        RootedWeightedNode<Weight = Self::Weight> + RootedMetaNode<'a, Meta = Self::Meta>,
{
    type Weight: Num + Clone + PartialOrd + NumCast + std::iter::Sum;
    type Meta: Display + Debug + Eq + PartialEq + Clone + Ord;

    fn from_newick(newick_str: &[u8]) -> Self;
    fn subtree_to_newick(&'a self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> impl Display;
    fn to_newick(&'a self) -> impl Display {
        format!("{};", self.subtree_to_newick(self.get_root_id()))
    }
}

// pub trait Nexus
// {
//     fn from_nexus(newick_str: &[u8])->impl RootedTree;
//     fn to_nexus(&self)->impl Display;
// }
