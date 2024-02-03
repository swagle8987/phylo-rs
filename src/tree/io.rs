use std::fmt::Display;

use super::simple_rtree::RootedTree;

pub trait Newick
{
    fn from_newick(newick_str: &[u8])->impl RootedTree;
    fn to_newick(&self)->impl Display;
}

pub trait Nexus
{
    fn from_newick(newick_str: &[u8])->impl RootedTree;
    fn to_newick(&self)->impl Display;
}