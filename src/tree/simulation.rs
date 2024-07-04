use crate::{node::simple_rnode::RootedMetaNode, tree::{RootedMetaTree, RootedTree}};
use anyhow::Result;

pub trait Yule<'a>: RootedMetaTree<'a>
where 
    <Self as RootedTree<'a>>::Node: RootedMetaNode<'a>,
{
    fn yule(num_taxa:usize)-> Result<Self>;
}

pub trait Uniform<'a>: RootedMetaTree<'a>
where 
    <Self as RootedTree<'a>>::Node: RootedMetaNode<'a>,
{
    fn unif(num_taxa:usize)-> Result<Self>;
}