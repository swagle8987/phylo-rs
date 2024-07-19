use std::fmt::Display;

use crate::tree::{RootedTree, TreeNodeID};

/// A trait descibing Newick encoding of a tree.
pub trait Newick<'a>: RootedTree<'a>
{
    /// Creates a new tree using a Newick string
    fn from_newick(newick_str: &[u8]) -> Self;

    /// Encodes a subtree starting from a node as a Newick string
    fn subtree_to_newick(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl Display;

    /// Encodes a tree as a Newick string
    fn to_newick(&self) -> impl Display {
        format!("{};", self.subtree_to_newick(self.get_root_id()))
    }
}