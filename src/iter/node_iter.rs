use itertools::Itertools;

use crate::{node::simple_rnode::RootedTreeNode, tree::simple_rtree::RootedTree};

pub trait DFS
where
    Self: RootedTree + Sized
{
    fn dfs(&self, start_node_id: Self::NodeID)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        let mut stack = vec![self.get_node(start_node_id).cloned().unwrap()];
        let mut out_vec = vec![];
        let mut visited = vec![];
        while let Some(x) = stack.pop()
        {
            match visited.contains(&x.get_id())
            {
                false => {
                    visited.push(x.get_id());
                    out_vec.push(x.clone());
                    for child_id in x.get_children(){
                        stack.push(self.get_node(child_id).cloned().unwrap());
                    }
                },
                true => {}
            };
        }
        out_vec
    }
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
    fn preord(&self, start_node_id: Self::NodeID)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        let mut stack = vec![self.get_node(start_node_id).cloned().unwrap()];
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
    fn postord(&self, _start_node: Self::NodeID)-> impl IntoIterator<Item = <Self as RootedTree>::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        // let mut stack = vec![self.get_node(start_node).cloned().unwrap()];
        // let mut out_vec = vec![];
        // match stack.pop() {
        //     Some(node) => {
        //         out_vec.push(node.clone());
        //         let children: Vec<<Self as RootedTree<Self>>::Node> = self.get_node_children(node.get_id()).into_iter().collect();
        //         for child in children.into_iter().rev()
        //         {
        //             stack.push(child);
        //         }
        //     }
        //     None => {},
        // };
        // out_vec
        vec![]
    }
}

pub trait Ancestors
where
    Self: RootedTree + Sized
{
    fn ancestors(&self, start_node_id: Self::NodeID)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        let mut stack = vec![self.get_node(start_node_id).cloned().unwrap()];
        while let Some(x) = stack.pop().unwrap().get_parent(){
            stack.push(self.get_node(x).cloned().unwrap());
            stack.push(self.get_node(stack[0].get_parent().unwrap()).cloned().unwrap())
        }
        stack
    }
}

pub trait EulerTour
where
    Self: RootedTree + DFS + Sized
{
    fn euler_tour(&self, start_node_id: Self::NodeID)->impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        let dfs = self.dfs(start_node_id.clone()).into_iter();
        let mut visited = vec![];
        let mut out_vec = vec![];
        for node in dfs
        {
            match visited.contains(&node.get_id())
            {
                true => {
                    match node.get_parent(){
                        Some(parent_id) => {out_vec.push(self.get_node(parent_id).cloned().unwrap())},
                        None => {},
                    };
                },
                false => {
                    visited.push(node.get_id());
                    out_vec.push(node.clone());
                }
            }
        }
        out_vec
    }
}

pub trait Clusters
where
    Self: RootedTree + Sized
{
    fn get_cluster(&self, node_id: Self::NodeID)-> impl IntoIterator<Item=Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>;
    fn get_bipartition(&self, edge: (Self::NodeID, Self::NodeID))->(impl IntoIterator<Item=Self::Node>, impl IntoIterator<Item=Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>);
}