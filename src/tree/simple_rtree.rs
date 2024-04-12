// use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Debug};
use std::hash::Hash;
use itertools::Itertools;
use num::{Num, NumCast};

use crate::node::simple_rnode::*;

pub trait RootedTree
{
    type NodeID: Display + Debug + Hash + Clone + Drop + Ord;
    type Node: RootedTreeNode<NodeID = Self::NodeID>;

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

    fn clean(&mut self);

    fn get_mrca(&self, node_id_list: &Vec<Self::NodeID>)->Self::NodeID;

    fn delete_edge(&mut self, parent_id: Self::NodeID, child_id: Self::NodeID)
    {
        self.get_node_mut(parent_id).unwrap().remove_child(&child_id);
        self.get_node_mut(child_id).unwrap().set_parent(None);
    }

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

    fn get_node_parent_id(&self, node_id: Self::NodeID)->Option<Self::NodeID>
    {
        self.get_node(node_id).unwrap().get_parent()
    }

    fn get_node_parent(&self, node_id: Self::NodeID)-> Option<Self::Node>
    {
        match self.get_node_parent_id(node_id)
        {
            Some(id) => self.get_node(id).cloned(),
            None => None
        }
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
        while let Some(parent_id) = self.get_node_parent_id(start_id)
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

    fn supress_node(&mut self, node_id: Self::NodeID)
    {
        let node_children_ids = self.get_node_children(node_id.clone()).into_iter().map(|x| x.get_id());
        let parent_id = self.get_node_parent_id(node_id.clone());
        

    }

    fn supress_unifurcations(&mut self)
    {
        todo!()
    }
}


pub trait RootedMetaTree: RootedTree
where
    Self::NodeID: Display + Debug + Hash + Clone + Ord,
    <Self as RootedTree>::Node : RootedMetaNode<Taxa = <Self as RootedMetaTree>::Taxa>
{    
    type Taxa: Display + Debug + Eq + PartialEq + Clone + Ord;

    fn get_taxa_node(&self, taxa: &Self::Taxa)->Option<Self::Node>
    {
        for node in self.get_nodes().into_iter()
        {
            if Some(taxa)==node.get_taxa().as_ref(){
                return Some(node);
            }
        }
        None
    }

    fn get_taxa_node_id(&self, taxa: &Self::Taxa)->Option<Self::NodeID>
    {
        match self.get_taxa_node(taxa){
            Some(node) => {return Some(node.get_id())},
            None => None
        }
    }
    fn num_taxa(&self)->usize
    {
        self.get_nodes().into_iter().filter(|f| f.is_leaf()).collect_vec().len()
    }
    fn set_node_taxa(&mut self, node_id: Self::NodeID, taxa: Option<Self::Taxa>)
    {
        self.get_node_mut(node_id).unwrap().set_taxa(taxa)
    }
    fn get_node_taxa(&self, node_id: Self::NodeID)->Option<Self::Taxa>
    {
        self.get_node(node_id).unwrap().get_taxa()
    }
    fn get_taxa_space(&self)->impl IntoIterator<Item=Self::Taxa, IntoIter = impl ExactSizeIterator<Item = Self::Taxa>>
    {
        self.get_nodes().into_iter().filter(|x| x.get_taxa().is_some()).map(|x| x.get_taxa().unwrap()).collect_vec()
    }
}

pub trait RootedWeightedTree: RootedTree
where
    Self::NodeID: Display + Debug + Hash + Clone + Ord,
    <Self as RootedTree>::Node : RootedWeightedNode<Weight = Self::Weight>
{
    type Weight: Num + Clone + PartialOrd + NumCast + std::iter::Sum;

    fn unweight(&mut self)
    {
        let node_ids = self.get_node_ids().into_iter().collect_vec();
        for node_id in node_ids
        {
            self.get_node_mut(node_id).unwrap().set_weight(None);
        }
    }
    
    fn set_edge_weight(&mut self, edge:(Self::NodeID, Self::NodeID), edge_weight:Option<Self::Weight>)
    {
        self.get_node_mut(edge.1).unwrap().set_weight(edge_weight);
    }

    fn is_weighted(&self)->bool
    {
        for node_id in self.get_node_ids()
        {
            if self.get_node(node_id).unwrap().get_weight()==None{
                return false;
            }
        }
        true
    }

    fn get_edge_weight(&self, _parent_id: Self::NodeID, child_id:Self::NodeID)->Option<Self::Weight>
    {
        self.get_node(child_id).unwrap().get_weight()
    }
}