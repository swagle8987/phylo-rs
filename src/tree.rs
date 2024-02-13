pub mod simple_rtree;
pub mod ops;
pub mod distances;
pub mod weighted;
pub mod io;


use std::collections::HashMap;
// use std::Arc::Arc;
use std::sync::Arc;

use itertools::Itertools;

use crate::node::simple_rnode::{RootedTreeNode, RootedPhyloNode, WeightedNode};
use crate::node::{Node, NodeID};
use crate::tree::simple_rtree::*;
use crate::iter::node_iter::*;

use self::io::Newick;
use self::ops::{Balance, Subtree, SPR};
use self::weighted::WeightedTree;

#[derive(Debug)]
pub struct SimpleRootedTree{
    root: NodeID,
    nodes: HashMap<NodeID, Node>,
}

impl SimpleRootedTree{
    pub fn next_id(&self)->NodeID
    {
        match self.nodes.keys().map(|x| (*Arc::clone(x)).clone()).max()
        {
            Some(x) => Arc::new(x+1),
            None => Arc::new(0)
        }
    }

    pub fn next_node(&self)->Node
    {
        Node::new(self.next_id(), false)
    }
}


impl RootedTree for SimpleRootedTree{
    
    type NodeID = NodeID;
    // type Taxa = String;
    type Node = Node;

    fn new(root_id: Self::NodeID)->Self{
        let root_node = Node::new(root_id, false);
        SimpleRootedTree { 
            root: root_node.get_id(),
            nodes: HashMap::from([(root_node.get_id(), root_node)]),
        }
    }

    /// Returns reference to node by ID
    fn get_node(&self, node_id: Self::NodeID)->Option<&Self::Node>
    {
        self.nodes.get(&node_id)
    }

    fn get_node_mut(&mut self, node_id: Self::NodeID)->Option<&mut Self::Node>
    {
        self.nodes.get_mut(&node_id)
    }

    fn get_node_ids(&self)->impl IntoIterator<Item = Self::NodeID, IntoIter = impl ExactSizeIterator<Item = Self::NodeID>> 
    {
        self.nodes.clone().into_keys()    
    }

    fn get_nodes(&self)->impl IntoIterator<Item = Self::Node, IntoIter = impl ExactSizeIterator<Item = Self::Node>> 
    {
        self.nodes.clone().into_values()    
    }

    /// Returns reference to node by ID
    fn set_node(&mut self, node: Self::Node)
    {
        self.nodes.insert(node.get_id(), node);
    }

    fn add_child(&mut self, parent_id: Self::NodeID, child: Self::Node)
    {
        let new_child_id = child.get_id();
        self.set_node(child);
        self.get_node_mut(Arc::clone(&parent_id)).unwrap().add_child(Arc::clone(&new_child_id));
        self.get_node_mut(new_child_id).unwrap().set_parent(Some(parent_id));
    }

    /// Get root node ID
    fn get_root_id(&self)->Self::NodeID
    {
        Arc::clone(&self.root)
    }

    fn set_root(&mut self, node_id: Self::NodeID) 
    {
        self.root = Arc::clone(&node_id);
    }

    fn remove_node(&mut self, node_id: Self::NodeID)->Option<Self::Node>
    {
        match self.get_node_parent(node_id.clone())
        {
            Some(pid) => self.get_node_mut(pid).unwrap().remove_child(node_id.clone()),
            None => {},
        }
        self.nodes.remove(&node_id)
    }

    fn contains_node(&self, node_id: Self::NodeID)->bool
    {
        self.nodes.contains_key(&node_id)
    }

    fn delete_edge(&mut self, parent_id: Self::NodeID, child_id: Self::NodeID)
    {
        self.get_node_mut(parent_id).unwrap().remove_child(Arc::clone(&child_id));
        self.get_node_mut(child_id).unwrap().set_parent(None);
    }

    fn clean(&mut self)
    {
        let node_iter = self.get_nodes().into_iter().collect::<Vec<Self::Node>>();
        for node in node_iter.clone(){
            // remove root with only one child
            let node_id = node.get_id();
            if node.get_id()==self.get_root_id() && node.degree()<2{
                let new_root = self.get_root().get_children().into_iter().next().unwrap();
                self.set_root(new_root);
                self.get_node_mut(Arc::clone(&self.root)).unwrap().set_parent(None);
                self.remove_node(node_id.clone());
            }
            // remove nodes with only one child
            else if !node.is_leaf() && node.get_parent()!=None && node.degree()<3 {
                let parent_id = self.get_node_parent(node_id.clone());
                let child_id = node.get_children().into_iter().next().unwrap();
                self.get_node_mut(Arc::clone(&child_id)).unwrap().set_parent(parent_id.clone());
                self.get_node_mut(parent_id.unwrap()).unwrap().add_child(child_id);
                self.remove_node(node.get_id());
            }
            // Removing dangling references to pruned children
            for chid in node.get_children().into_iter()
            {
                if !node_iter.clone().into_iter().map(|x| x.get_id()).contains(&chid)
                {
                    self.get_node_mut(node_id.clone()).unwrap().remove_child(chid);
                }
            }
        }
    }

    fn get_mrca(&self, node_id_list: &Vec<Self::NodeID>)->Self::NodeID
    {
        let euler_tour = self.euler_tour(self.get_root_id()).into_iter().map(|x| x.get_id()).collect_vec();
        let depth_array: Vec<usize> = euler_tour.iter().map(|x| self.get_node_depth(x.clone())).collect_vec();   // todo
        let mut min_pos = euler_tour.len();
        let mut max_pos = 0;
        for node_id in node_id_list
        {
            let pos = euler_tour.iter().position(|r| r == node_id).unwrap();
            match pos<min_pos {
                true => min_pos=pos,
                false => {},
            }
            let pos = euler_tour.iter().rposition(|r| r == node_id).unwrap_or(0);
            match pos>max_pos {
                true => max_pos=pos,
                false => {},
            }
        }
        let depth_subarray_min_value = depth_array[min_pos..max_pos].iter().min().unwrap();
        let depth_subarray_min_pos = depth_array[min_pos..max_pos].iter().position(|x| x==depth_subarray_min_value).unwrap();
        Arc::clone(&euler_tour[min_pos..max_pos][depth_subarray_min_pos])
    }
}



impl WeightedTree for SimpleRootedTree {
    type EdgeWeight = f64;

    fn unweight(&mut self)
    {
        let node_ids = self.get_node_ids().into_iter().collect_vec();
        for node_id in node_ids
        {
            self.get_node_mut(node_id).unwrap().set_weight(None);
        }
    }

    fn set_edge_weight(&mut self, edge:(Self::NodeID, Self::NodeID), edge_weight:Option<Self::EdgeWeight>)
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

    fn get_edge_weight(&self, _parent_id: Self::NodeID, child_id:Self::NodeID)->Option<Self::EdgeWeight>
    {
        self.get_node(child_id).unwrap().get_weight()
    }
}

impl PreOrder for SimpleRootedTree{}

impl PostOrder for SimpleRootedTree{}

impl DFS for SimpleRootedTree{}

impl EulerTour for SimpleRootedTree{}

impl Newick for SimpleRootedTree{
    fn from_newick(newick_str: &[u8])->Self {
                let mut tree = SimpleRootedTree::new(Arc::new(0));
                let mut stack : Vec<NodeID> = Vec::new();
                let mut context : NodeID = tree.get_root_id();
                let mut taxa_str = String::new();
                let mut decimal_str: String = String::new();
                let mut str_ptr: usize = 0;
                let newick_string = String::from_utf8(newick_str.to_vec()).unwrap().chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>();
                while str_ptr<newick_string.len(){
                    match newick_string[str_ptr]{
                        '(' => {
                            stack.push(context);
                            let new_node = Node::new(tree.next_id(), false);
                            context = new_node.get_id();
                            tree.set_node(new_node);
                            str_ptr +=1;
                        },
                        ')'|',' => {
                            // last context id
                            let last_context = stack.last().expect("Newick string ended abruptly!");
                            // add current context as a child to last context
                            tree.set_child(
                                Arc::clone(last_context),
                                Arc::clone(&context),
                            );
                            tree.set_edge_weight((Arc::clone(last_context),Arc::clone(&context)), decimal_str.parse::<Self::EdgeWeight>().ok());
                            if !taxa_str.is_empty(){
                                tree.set_node_taxa(context, Some(taxa_str.to_string()));
                            }
                            // we clear the strings
                            taxa_str.clear();
                            decimal_str.clear();
        
                            match newick_string[str_ptr] {
                                ',' => {
                                    let new_node = Node::new(tree.next_id(), false);
                                    context = new_node.get_id();
                                    tree.set_node(new_node);
                                    str_ptr += 1;
                                }
                                _ => {
                                    context = stack.pop().expect("Newick string ended abruptly!");
                                    str_ptr += 1;
                                }
                            }
                        },
                        ';'=>{
                            if !taxa_str.is_empty(){
                                tree.set_node_taxa(context, Some(taxa_str));
                            }
                            break;
                        }
                        ':' => {
                                // if the current context had a weight
                                if newick_string[str_ptr]==':'{
                                    str_ptr+=1;
                                    while newick_string[str_ptr].is_ascii_digit() || newick_string[str_ptr]=='.'{
                                        decimal_str.push(newick_string[str_ptr]); 
                                        str_ptr+=1;
                                    }
                                }
                            }
                        _ => {
                            // push taxa characters into taxa string
                            while newick_string[str_ptr]!=':'&&newick_string[str_ptr]!=')'&&newick_string[str_ptr]!=','&&newick_string[str_ptr]!='('&&newick_string[str_ptr]!=';'{
                                taxa_str.push(newick_string[str_ptr]); 
                                str_ptr+=1;
                            }
                        },
                    }
                }
                let leaf_ids = tree.dfs(tree.get_root_id())
                    .into_iter()
                    .filter(|x| x.get_children().into_iter().collect_vec().is_empty())
                    .map(|x| Arc::clone(&x.get_id()))
                    .collect_vec();
                tree.flip_nodes(leaf_ids.clone().into_iter());
                tree
            }

    fn subtree_to_newick(&self, node_id: Self::NodeID)-> impl std::fmt::Display {
        let node  = self.get_node(node_id).unwrap();
        let mut tmp = String::new();
        if node.get_children().into_iter().len()!=0{
            if node.get_children().into_iter().len()>1{
                tmp.push('(');
            }
            for child_id in node.get_children().into_iter(){
                let child_str = format!("{},", self.subtree_to_newick(child_id));
                tmp.push_str(&child_str);
            }
            tmp.pop();
            if node.get_children().into_iter().collect_vec().len()>1{
                tmp.push(')');
            }
        }
        tmp.push_str(&node.get_taxa().unwrap_or("".to_string()));
        tmp

        // String::new()
    }
}

impl SPR for SimpleRootedTree
{
    fn graft(&mut self, tree: Self, edge: (Self::NodeID, Self::NodeID)) {
        let new_node = self.next_node();
        let new_node_id = dbg!(new_node.get_id());
        for node in tree.dfs(tree.get_root_id())
        {
            self.set_node(dbg!(node));
        }
        self.split_edge(edge, new_node);
        self.set_child(dbg!(new_node_id), dbg!(tree.get_root_id()));

    }
    fn prune(&mut self, node_id: Self::NodeID)-> Self {
        let mut pruned_tree = SimpleRootedTree::new(Arc::clone(&node_id));
        let p_id = self.get_node_parent(Arc::clone(&node_id)).unwrap();
        self.get_node_mut(p_id).unwrap().remove_child(Arc::clone(&node_id));
        pruned_tree.get_node_mut(pruned_tree.get_root_id()).unwrap().add_children(self.get_node(Arc::clone(&node_id)).unwrap().get_children());
        let dfs = self.dfs(node_id.clone()).into_iter().collect_vec().clone();
        for node in dfs
        {    
            self.nodes.remove(&node.get_id());
            pruned_tree.set_node(node);
        }
        pruned_tree
    }
}

impl Clusters for SimpleRootedTree{}

impl Balance for SimpleRootedTree{
    fn balance_subtree(&mut self) 
    {
        assert!(self.get_cluster(self.get_root_id()).into_iter().collect_vec().len()==4, "Quartets have 4 leaves!");
        assert!(self.is_binary(), "Cannot balance non-binary tree!");
        let root_children = self.get_node_children(self.get_root_id()).into_iter().collect_vec();
        let (child1, child2) = (root_children[0].get_id(), root_children[1].get_id());
        let next_id = self.next_id();
        let split_id = Arc::new(*self.next_id()+1);
        match dbg!(((self.get_node(child1.clone()).unwrap().is_leaf()), (self.get_node(child2.clone()).unwrap().is_leaf()))){
            (false, false) => {},
            (true, false) => {
                let mut leaf_node = self.remove_node(child1).unwrap();
                leaf_node.set_id(next_id.clone());
                let other_leaf_id = &self.get_node_children(child2.clone()).into_iter().filter(|node| node.is_leaf()).collect_vec()[0].get_id();
                self.split_edge((child2, Arc::clone(other_leaf_id)), Node::new(split_id.clone(), false));
                self.add_child(dbg!(split_id), leaf_node);
            },
            (false, true) => {
                let mut leaf_node = self.remove_node(child2).unwrap();
                leaf_node.set_id(next_id.clone());
                let other_leaf_id = &self.get_node_children(child1.clone()).into_iter().filter(|node| node.is_leaf()).collect_vec()[0].get_id();
                self.split_edge((child1, Arc::clone(other_leaf_id)), Node::new(split_id.clone(), false));
                self.add_child(split_id, leaf_node);
            },
            _ =>{}
        }
        self.clean();
    }
}

impl RootedPhyloTree for SimpleRootedTree{
    type Node = Node;
    type Taxa = String;

    fn get_taxa_id(&self, taxa: &Self::Taxa)->Option<Self::NodeID>
    {
        for node in self.get_nodes().into_iter()
        {
            if Some(taxa)==node.get_taxa().as_ref(){
                return Some(node.get_id());
            }
        }
        None
    }
    fn num_taxa(&self)->usize
    {
        self.get_nodes().into_iter().filter(|f| f.is_leaf()).collect_vec().len()
    }
    fn set_node_taxa(&mut self, node_id: Self::NodeID, taxa: Option<Self::Taxa>) 
    {
        self.get_node_mut(node_id).unwrap().set_taxa(taxa)
    }
}

impl Ancestors for SimpleRootedTree{}

impl Subtree for SimpleRootedTree {}