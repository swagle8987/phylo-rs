use std::{collections::VecDeque, vec};

use itertools::Itertools;

use crate::{node::simple_rnode::RootedTreeNode, tree::simple_rtree::RootedTree};

pub trait DFS
where
    Self: RootedTree + Sized
{
    fn dfs(&self, start_node_id: Self::NodeID)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        let mut stack = VecDeque::from([self.get_node(start_node_id).cloned().unwrap()]);
        let mut out_vec = vec![];
        let mut visited = vec![];
        while let Some(x) = stack.pop_front()
        {
            match visited.contains(&x.get_id())
            {
                false => {
                    visited.push(x.get_id());
                    out_vec.push(x.clone());
                    for child_id in x.get_children().into_iter().collect_vec().into_iter().rev(){
                        stack.push_front(self.get_node(child_id).cloned().unwrap());
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
        let mut stack = VecDeque::from([self.get_node(start_node_id).cloned().unwrap()]);
        let mut out_vec = vec![];
        let mut visited = vec![];
        while let Some(x) = stack.pop_front()
        {
            match visited.contains(&x.get_id())
            {
                false => {
                    visited.push(x.get_id());
                    out_vec.push(x.clone());
                    for child_id in x.get_children().into_iter().collect_vec().into_iter().rev(){
                        stack.push_front(self.get_node(child_id).cloned().unwrap());
                    }
                },
                true => {}
            };
        }
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
        let mut stack = VecDeque::from([self.get_node(start_node_id).cloned().unwrap()]);
        while let Some(x) = stack.pop_front(){
            stack.push_front(x.clone());
            match x.get_parent()
            {
                Some(pid) => {stack.push_front(self.get_node(pid).cloned().unwrap());},
                None => {break;},
            }
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
        let mut stack = VecDeque::from([self.get_node(start_node_id).unwrap()]);
        let mut visited = vec![];
        let mut out_vec = vec![];
        while let Some(node) = stack.pop_front()
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
                    stack.push_front(node);
                    for child_id in node.get_children().into_iter().collect_vec().into_iter().rev(){
                        stack.push_front(self.get_node(child_id).unwrap())
                    }
                }
            }
        }
        out_vec
    }
}

pub trait Clusters
where
    Self: DFS + Sized
{
    fn get_cluster(&self, node_id: Self::NodeID)-> impl IntoIterator<Item=Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        self.dfs(node_id).into_iter().filter(|x| x.is_leaf()).collect_vec()
    }
    fn get_bipartition(&self, edge: (Self::NodeID, Self::NodeID))->(impl IntoIterator<Item=Self::Node>, impl IntoIterator<Item=Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>)
    {
        let c2 = self.get_cluster(edge.1.clone());
        let c2_ids = self.get_cluster(edge.1).into_iter().map(|x| x.get_id()).collect_vec();
        let c1 = self.get_cluster(edge.0).into_iter().filter(|x| !c2_ids.contains(&x.get_id())).collect_vec();
        (c1,c2)
    }
}