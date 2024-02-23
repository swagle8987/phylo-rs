// use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Debug};
use std::hash::Hash;
use itertools::Itertools;

use crate::node::simple_rnode::*;

pub trait RootedTree<RHS=Self> {
    type NodeID: Display + Debug + Hash + Drop + Clone + Ord;
    type Node: RootedTreeNode<NodeID = Self::NodeID> + Debug + Clone;

    fn new(root_id: Self::NodeID)->Self;

    /// Returns reference to node by ID
    fn get_node(&self, node_id: Self::NodeID)->Option<&Self::Node>;

    fn get_node_mut(&mut self, node_id: Self::NodeID)->Option<&mut Self::Node>;

    fn get_node_ids(&self)->impl IntoIterator<Item = Self::NodeID, IntoIter = impl ExactSizeIterator<Item = Self::NodeID>>;

    fn get_nodes(&self)->impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>;

    fn get_root_id(&self)->Self::NodeID;

    fn set_root(&mut self, node_id: Self::NodeID);

    /// Returns reference to node by ID
    fn set_node(&mut self, node: Self::Node);

    fn add_child(&mut self, parent_id: Self::NodeID, child: Self::Node);

    fn remove_node(&mut self, node_id: Self::NodeID)->Option<Self::Node>;

    fn contains_node(&self, node_id: Self::NodeID)->bool;

    fn delete_edge(&mut self, parent_id: Self::NodeID, child_id: Self::NodeID)
    {
        self.get_node_mut(parent_id).unwrap().remove_child(&child_id);
        self.get_node_mut(child_id).unwrap().set_parent(None);
    }

    fn clean(&mut self);

    fn get_mrca(&self, node_id_list: &Vec<Self::NodeID>)->Self::NodeID;

    fn set_nodes(&mut self, node_list: impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>)
    {
        for node in node_list.into_iter()
        {
            self.set_node(node);
        }
    }

    fn split_edge(&mut self, edge: (Self::NodeID, Self::NodeID), node: Self::Node)
    {
        let p_id = edge.0;
        let c_id = edge.1;
        let n_id = node.get_id();
        self.set_node(node);
        self.get_node_mut(p_id.clone()).unwrap().remove_child(&c_id);
        self.set_child(p_id.clone(), n_id.clone());
        self.set_child(n_id, c_id);

    }

    /// Get root node
    fn get_root(&self)->&Self::Node
    {
        self.get_node(self.get_root_id()).unwrap()
    }

    fn set_child(&mut self, parent_id: Self::NodeID, child_id: Self::NodeID)
    {
        self.get_node_mut(parent_id.clone()).unwrap().add_child(child_id.clone());
        self.get_node_mut(child_id).unwrap().set_parent(Some(parent_id));
    }

    fn get_node_parent(&self, node_id: Self::NodeID)->Option<Self::NodeID>
    {
        self.get_node(node_id).unwrap().get_parent()
    }

    fn get_node_children(&self, node_id: Self::NodeID)->impl IntoIterator<Item=Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>>
    {
	let node = self.get_node(node_id).unwrap();
	node.get_children()
	    .into_iter()
	    .map(|x| self.get_node(x).cloned().unwrap())
	    .collect::<Vec<Self::Node>>()
    }

    fn node_degree(&self, node_id: Self::NodeID)->usize
    {
        self.get_node(node_id).unwrap().degree()
    }
    fn get_node_depth(&self, node_id: Self::NodeID)->usize
    {
        let mut start_id = node_id;
        let mut depth = 0;
        while let Some(parent_id) = self.get_node_parent(start_id)
        {
            depth+=1;start_id=parent_id;
        };
        depth
    }
    fn is_binary(&self)->bool
    {
        for node in self.get_nodes()
        {
            if node.get_children().into_iter().collect_vec().len()>2
            {
                return false;
            }
        }
        true
    }
}


pub trait RootedPhyloTree<RHS=Self>
where
    Self: RootedTree + Sized
{
    type Taxa: Debug + Eq + PartialEq + Clone + Ord;
    type Node: RootedPhyloNode<Taxa = Self::Taxa> + Debug + Clone;
    fn get_taxa_id(&self, taxa: &Self::Taxa)->Option<Self::NodeID>;
    fn num_taxa(&self)->usize;
    fn set_node_taxa(&mut self, node_id: Self::NodeID, taxa: Option<Self::Taxa>);
}
