use itertools::Itertools;

use crate::{node::simple_rnode::RootedTreeNode, tree::simple_rtree::RootedTree};

pub trait DFS
where
    Self: RootedTree + Sized
{
    fn dfs(&self)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>;
}

pub trait BFS
where
    Self: RootedTree + Sized
{
    fn bfs(&self)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>;
}

pub trait PreOrder
where
    Self: RootedTree + Sized
{
    fn preord(&self, start_node: Self::NodeID)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        let mut stack = vec![self.get_node(start_node).cloned().unwrap()];
        let mut out_vec = vec![];
        match stack.pop() {
            Some(node) => {
                let children = self.get_node_children(node.get_id()).into_iter().collect_vec();
                for child in children.into_iter().rev()
                {
                    stack.push(child)
                }
                out_vec.push(node.clone());
            }
            None => {},
        };
        out_vec
    }
}

pub trait PostOrder
where
    Self: RootedTree + Sized
{
    fn postord(&self, start_node: Self::NodeID)-> impl IntoIterator<Item = <Self as RootedTree>::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        let mut stack = vec![self.get_node(start_node).cloned().unwrap()];
        let mut out_vec = vec![];
        match stack.pop() {
            Some(node) => {
                let children: Vec<<Self as RootedTree<Self>>::Node> = self.get_node_children(node.get_id()).into_iter().collect();
                for child in children.into_iter().rev()
                {
                    stack.push(child);
                }
                out_vec.push(node.clone());
            }
            None => {},
        };
        out_vec
    }
}

pub trait Ancestors
where
    Self: RootedTree + Sized
{
    fn ancestor(&self)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>;
}

pub trait Clusters
where
    Self: RootedTree + Sized
{
    fn get_cluster(&self, node_id: Self::NodeID)-> impl IntoIterator<Item=Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>;
    fn get_bipartition(&self, edge: (Self::NodeID, Self::NodeID))->(impl IntoIterator<Item=Self::Node>, impl IntoIterator<Item=Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>);
}