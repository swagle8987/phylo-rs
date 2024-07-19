use crate::{node::simple_rnode::RootedMetaNode, tree::{RootedMetaTree, RootedTree}};
use anyhow::Result;

/// A trait describing generation of a random binary tree under the Yule model.
pub trait Yule<'a>: RootedMetaTree<'a>
where 
    <Self as RootedTree<'a>>::Node: RootedMetaNode<'a>,
{
    /// Generate a random binary tree under the Yule model with num_taxa
    fn yule(num_taxa:usize)-> Result<Self>;
}

/// A trait describing generation of a random binary tree under the Uniform model.
pub trait Uniform<'a>: RootedMetaTree<'a>
where 
    <Self as RootedTree<'a>>::Node: RootedMetaNode<'a>,
{
    /// Generate a random binary tree under the Uniform model with num_taxa
    fn unif(num_taxa:usize)-> Result<Self>;
}