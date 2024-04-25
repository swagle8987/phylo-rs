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
    fn postord(&self, _start_node: Self::NodeID)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
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
    fn root_to_node(&self, start_node_id: Self::NodeID)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
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

    fn node_to_root(&self, start_node_id: Self::NodeID)-> impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
        let mut stack = VecDeque::from([self.get_node(start_node_id).cloned().unwrap()]);
        while let Some(x) = stack.pop_front(){
            stack.push_back(x.clone());
            match x.get_parent()
            {
                Some(pid) => {stack.push_front(self.get_node(pid).cloned().unwrap());},
                None => {break;},
            }
        }
        stack
    }

    fn depth(&self, node_id: Self::NodeID)->usize
    {
        self.node_to_root(node_id).into_iter().len()-1
    }
}

pub trait EulerWalk
where
    Self: RootedTree + Sized
{
    fn euler_walk(&self, start_node_id: Self::NodeID)->impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
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

    fn get_lca_id(&self, node_id_1: Self::NodeID, node_id_2: Self::NodeID, walk: Option<Vec<Self::NodeID>>)-> Self::NodeID
    {
        let euler_walk = match walk{
            None => self.euler_walk(self.get_root_id()).into_iter().map(|x| x.get_id()).collect_vec(),
            Some(_) => walk.unwrap(),
        };
        let depth_array: Vec<usize> = euler_walk.iter().map(|x| self.get_node_depth(x.clone())).collect_vec();   // todo
        let mut min_pos = euler_walk.len();
        let mut max_pos = 0;
        let node_id_list = vec![node_id_1, node_id_2];
        for node_id in node_id_list
        {
            let pos = euler_walk.iter().position(|r| r == &node_id).unwrap();
            match pos<min_pos {
                true => min_pos=pos,
                false => {},
            }
            let pos = euler_walk.iter().rposition(|r| r == &node_id).unwrap_or(0);
            match pos>max_pos {
                true => max_pos=pos,
                false => {},
            }
        }
        let depth_subarray_min_value = depth_array[min_pos..max_pos].iter().min().unwrap();
        let depth_subarray_min_pos = depth_array[min_pos..max_pos].iter().position(|x| x==depth_subarray_min_value).unwrap();
        euler_walk[min_pos..max_pos][depth_subarray_min_pos].clone()
    }

    fn get_lca(&self, node_id_1: Self::NodeID, node_id_2: Self::NodeID, walk: Option<Vec<Self::NodeID>>)-> Self::Node
    {
        self.get_node(self.get_lca_id(node_id_1, node_id_2, walk)).cloned().unwrap()
    }
}

pub trait Clusters: DFS + Sized
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

    fn get_median_node(&self)->Self::Node
    {
        let mut median_node: Self::Node = self.get_root().clone();
        let num_leaves = self.get_cluster(self.get_root_id()).into_iter().len();
        while self.get_cluster(median_node.get_id()).into_iter().len()>(num_leaves/2){
            median_node = self.get_node_children(median_node.get_id()).into_iter().max_by(|x, y| self.get_cluster(x.get_id()).into_iter().len().cmp(&self.get_cluster(y.get_id()).into_iter().len())).unwrap();
        }
        median_node
    }

    fn get_median_node_id(&self)->Self::NodeID
    {
        self.get_median_node().get_id()
    }
}