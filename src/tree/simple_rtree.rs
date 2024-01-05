use std::collections::HashMap;
use std::slice::Iter;
use itertools::Itertools;
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::rc::Rc;

use crate::node::simple_rnode::*;

pub trait SimpleRTree<RHS=Self> 
{
    type NodeID: Display + Debug + Hash + Copy + Clone + Ord + Add<Output = Self::NodeID> + AddAssign + Sub<Output = Self::NodeID> + SubAssign;
    type EdgeWeight: Display + Debug + Clone + Add<Output = Self::EdgeWeight> + AddAssign + Sub<Output = Self::EdgeWeight> + SubAssign;
    type Taxa: Display + Debug + Clone + Ord;
    type Node: RootedTreeNode<NodeID = Self::NodeID, Taxa = Self::Taxa, Weight = Self::EdgeWeight>;

    type Nodes: Debug + Iterator + IntoIterator<Item=(Rc<Self::NodeID>, Self::Node)>; //+ Index<&Rc<Self::NodeID>, Output=Self::Node> ;

    /// Creates a new node in the tree and returns new node ID.
    fn create_node(&mut self)->Rc<Self::NodeID>;

    /// Assign taxa to leaf node
    fn assign_taxa(&mut self, node_id:Rc<Self::NodeID>, taxa:Option<Self::Taxa>);

    /// Get root node ID
    fn get_root_id(&self)->Rc<Self::NodeID>;
    
    /// Returns all node ids
    fn get_nodes(&self)->&Self::Nodes;

    /// Returns reference to node by ID
    fn get_node(&self, node_id: Rc<Self::NodeID>)->Option<&Self::Node>;    

    /// Returns mutable refence to node by ID
    fn get_node_mut(&mut self, node_id: Rc<Self::NodeID>)->Option<&mut Self::Node>;    
    
    /// Returns full subtree rooted at given node
    fn get_subtree(&self, node_id: Rc<Self::NodeID>)->Box<RHS>;
    
    /// Returns most recent common ancestor of give node set
    fn get_mrca(&self, node_id_list: &Vec<Rc<Self::NodeID>>)->Rc<Self::NodeID>;

    /// Attaches input tree to self by spliting an edge
    fn graft(&mut self, tree: Box<RHS>, edge: (Rc<Self::NodeID>, Rc<Self::NodeID>), edge_weights:(Option<Self::EdgeWeight>, Option<Self::EdgeWeight>), graft_edge_weight: Option<Self::EdgeWeight>);
    
    /// Returns subtree starting at given node, while corresponding nodes from self.
    fn prune(&mut self, node_id: Rc<Self::NodeID>)-> Box<RHS>;

    ///Returns an iterator that iterates over the nodes in Pre-order
    fn iter_nodes_pre(&self, start_node_id: Rc<Self::NodeID>)->Iter<Rc<Self::NodeID>>;
    
    ///Returns an iterator that iterates over the nodes in Post-order
    fn iter_nodes_post(&self, start_node_id: Rc<Self::NodeID>)->Iter<Rc<Self::NodeID>>;
    
    ///Returns an iterator that iterates over the edges in Pre-order
    fn iter_edges_pre(&self, start_node_id: Rc<Self::NodeID>)->Iter<(Rc<Self::NodeID>, Rc<Self::NodeID>, Option<&Self::EdgeWeight>)>;
    
    ///Returns an iterator that iterates over the edges in Post-order
    fn iter_edges_post(&self, start_node_id: Rc<Self::NodeID>)->Iter<(Rc<Self::NodeID>, Rc<Self::NodeID>, Option<&Self::EdgeWeight>)>;

    /// Returns all node ids in path from root to given node
    fn get_ancestors_pre(&self, node_id: Rc<Self::NodeID>)->Iter<Rc<Self::NodeID>>;

    /// Rerootes tree at given node.
    fn reroot_at_node(&mut self, node_id: Rc<Self::NodeID>);

    /// Returns distance of node from some ancestor of node, returns None is ancestor_id==node_id. If weighted is true, it returns sum of edges from root to self.
    fn distance_from_ancestor(&self, node_id: Rc<Self::NodeID>, ancestor_id: Rc<Self::NodeID>, weighted: bool)->Self::EdgeWeight;

    /// Returns cluster of node
    fn get_cluster(&self, node_id: Rc<Self::NodeID>)-> Iter<Self::NodeID>;

    /// Checks if tree is binary
    fn is_binary(&self)->bool
    {
        todo!()
    }

    /// Returns bipartition induced by edge
    fn get_bipartition(&self, edge: (Rc<Self::NodeID>, Rc<Self::NodeID>))->(Vec<Rc<Self::NodeID>>, Vec<Rc<Self::NodeID>>)
    {
        // This can be done using get_cluster
        todo!()
    }

    // removes half-nodes in a subtree starting at input node and returns id of new root node of subtree and in-going edge weight
    fn clean_subtree(&mut self, node_id: Self::NodeID)->(Rc<Self::NodeID>, Option<Self::EdgeWeight>)
    {
        todo!()
    }

    /// Cleans self by removing 1) half-nodes, 2) Floating nodes, 3) self loops
    fn clean(&mut self)
    {
        // can be implemented using clean_subtree
        todo!()
    }

    // This may not be needed, commenting out for now
    // /// Increment all node_ids
    // fn incerement_ids(&mut self, value: Rc<Self::NodeID>);

    /// Remove all weights
    fn unweight(&mut self)
    {
        for id in self.iter_nodes_pre(self.get_root_id()){
            self.get_node_mut(id).unwrap().unweight();
        }
    }

    /// Returns root node id
    fn get_root(&self)->&Self::Node
    {
        self.get_node(self.get_root_id()).expect("No such node exists!")
    }

    /// Sets node_id as child to parent.
    fn set_child(&mut self, parent_id:Rc<Self::NodeID>, child_id:Rc<Self::NodeID>)
    {
        self.get_node_mut(parent_id).expect("No node with id {parent_id} exists!").add_child(child_id);
    }

    /// Converts internal node to leaf_node
    fn set_leaf(&mut self, node_id: Rc<Self::NodeID>)
    {
        match self.is_leaf(node_id){
            true => {},
            false => {self.get_node_mut(node_id).unwrap().flip()},
        };
    }

    /// Sets the edge weight between two nodes (None to unweight the edge)
    fn set_edge_weight(&mut self, edge:(Rc<Self::NodeID>, Rc<Self::NodeID>), edge_weight:Option<Self::EdgeWeight>)
    {
        self.get_node_mut(edge.1).expect("No such edge exists!").set_weight(edge_weight);
    }

    /// Returns children node ids for given node id 
    fn get_node_children(&self, node_id: Rc<Self::NodeID>)->Vec<Rc<Self::NodeID>>
    {
        self.get_node(node_id).expect("No such node exists!").get_children().clone()
    }

    /// Returns node parent
    fn get_node_parent(&self, node_id:Rc<Self::NodeID>)->Option<Rc<Self::NodeID>>
    {
        self.get_node(node_id).expect("No such node exists!").get_parent().cloned()
    }

    /// Get node taxa
    fn get_taxa(&self, node_id:Rc<Self::NodeID>)->Option<Self::Taxa>
    {
        self.get_node(node_id).expect("No such node exists!").get_taxa().cloned()
    }
    
    /// Checks if the given node is a leaf node
    fn is_leaf(&self, node_id: Rc<Self::NodeID>)->bool{
        self.get_node(node_id).expect("No such node exists!").is_leaf()
    }

    /// Inserts node in the middle of edge given by pair of node ids, and returns the new node id
    fn split_edge(&mut self, edge: (Rc<Self::NodeID>, Rc<Self::NodeID>), edge_weights:(Option<Self::EdgeWeight>, Option<Self::EdgeWeight>))->Rc<Self::NodeID>
    {
        // Create a new node
        let new_node_id = self.create_node();

        // Set new node as child of edge.0 and remove edge.1 from children or edge.0
        self.set_child(edge.0, Rc::clone(&new_node_id));
        self.set_edge_weight((edge.0, Rc::clone(&new_node_id)), edge_weights.0);
        self.get_node_mut(edge.0).unwrap().remove_child(edge.1);

        // Add edge.1 as child of new node and set new node as parent of edge.1
        self.get_node_mut(Rc::clone(&new_node_id)).unwrap().add_child(edge.1);
        self.get_node_mut(edge.1).unwrap().set_parent(Rc::clone(&new_node_id));

        // return new node id
        new_node_id
    }    
    
    /// Returns all leaf node ids
    fn get_leaves(&self)->Iter<Rc<Self::NodeID>>
    {
        self.get_cluster(self.get_root_id())
    }
 

    /// Sets iterable of node_ids as children to parent
    fn add_children(&mut self, parent: Rc<Self::NodeID>, children: &[Rc<Self::NodeID>]){
        for child_id in children.into_iter(){
            self.set_child(Rc::clone(&parent), Rc::clone(child_id));
        }
    }

    /// Returns node degree
    fn get_node_degree(&self, node_id:Rc<Self::NodeID>)->usize{
        self.get_node_children(Rc::clone(&node_id)).into_iter().collect_vec().len() + match self.get_node_parent(node_id) {
            Some(_) => 1,
            None => 0
        }
    }

    /// Check if tree is weighted
    fn is_weighted(&self)->bool{
        for (_, _, weight) in self.iter_edges_post(self.get_root_id()){
            match weight{
                    None => {},
                    Some(_) => {return true}
                    }
            }
        false
    }

    /// Returns pairwise distance matrix of the taxa. If weighted is true, then returns sum of edge weights along paths connecting leaves of tree
    fn leaf_distance_matrix(&self, weighted: bool)->HashMap<(Self::NodeID, Self::NodeID), Self::EdgeWeight>{
        todo!()
        // let binding = self.get_leaves(self.get_root());
        // let leaves = binding.iter().map(|leaf_id| leaf_id.clone()).combinations(2);
        // let mut dist_mat: HashMap<(Self::NodeID, Self::NodeID), Self::EdgeWeight> = HashMap::new();
        // for node_pair in leaves{
        //     let w  = self.distance_from_node(&node_pair[0], &node_pair[1], weighted);
        //     dist_mat.insert((node_pair[0].clone(), node_pair[1].clone()), w);
        // }
        // dist_mat
    }

    /// Returns pairwise distance matrix of all nodes. If weighted is true, then returns sum of edge weights along paths connecting leaves of tree
    fn node_distance_matrix(&self, weighted: bool)->HashMap<(Self::NodeID, Self::NodeID), Self::EdgeWeight>{
        todo!()
        // let binding = self.get_nodes().clone();
        // let leaves = binding.into_iter().map(|(id, _)| id).combinations(2);
        // let mut dist_mat: HashMap<(Self::NodeID, Self::NodeID), Self::EdgeWeight> = HashMap::new();
        // for node_pair in leaves{
        //     let w  = self.distance_from_node(&node_pair[0], &node_pair[1], weighted);
        //     dist_mat.insert((node_pair[0], node_pair[1]), w);
        // }
        // dist_mat
    }
    
    /// Rerootes tree at edge.
    fn reroot_at_edge(&mut self, edge: (Rc<Self::NodeID>, Rc<Self::NodeID>), edge_weights: (Option<Self::EdgeWeight>, Option<Self::EdgeWeight>)){
        let split_node_id = self.split_edge(edge, edge_weights).clone();
        self.reroot_at_node(split_node_id);
    }

    /// Returns distance of node from root. If weighted is true, it returns sum of edges from root to self.
    fn distance_from_root(&self, node_id: Rc<Self::NodeID>, weighted: bool)->Self::EdgeWeight
    {
        self.distance_from_ancestor(node_id, self.get_root_id(), weighted)
    }

    /// Returns distance of node from root. If weighted is true, it returns sum of edges from root to self.
    fn distance_from_node(&self, node1: Rc<Self::NodeID>, node2: Rc<Self::NodeID>, weighted: bool)->Self::EdgeWeight{
        let mrca = self.get_mrca(&vec![Rc::clone(&node1), Rc::clone(&node1)]);
        
        self.distance_from_ancestor(Rc::clone(&node1), Rc::clone(&mrca), weighted) + self.distance_from_ancestor(node2, mrca, weighted)
    }

    /// Get edge weight
    fn get_edge_weight(&self, _parent_id: Rc<Self::NodeID>, _child_id:Rc<Self::NodeID>)->Option<Self::EdgeWeight>{
        todo!()
        // self.get_node(child_id).get_weight()
    }

    /// return subtree as newick string; utf-8 encoded slice
    fn subtree_to_newick(&self, _node_id:Rc<Self::NodeID>)->String{
        todo!()
        // fn print_node<EdgeWeight: Display + Debug + Clone + Add<Output=EdgeWeight> + AddAssign + Sub<Output=EdgeWeight> + SubAssign, Node: RNode>(node: &Node)->String{
        //     match node.get_weight() {
        //         Some(w) => format!("{}:{}", node.get_taxa(), w),
        //         None => node.get_taxa()
        //     }
        // }

        // let node  = self.get_node(node_id);
        // let mut tmp = String::new();
        // if !self.get_node_children(node_id).clone().into_iter().collect_vec().len()==0{
        //     if self.get_node_children(node_id).clone().into_iter().collect_vec().len()>1{
        //         tmp.push('(');
        //     }
        //     for child_id in self.get_node_children(node_id).clone().into_iter(){
        //         let child_str = format!("{},", self.subtree_to_newick(&child_id));
        //         tmp.push_str(&child_str);
        //     }
        //     tmp.pop();
        //     if self.get_node_children(node_id).clone().into_iter().collect_vec().len()>1{
        //         tmp.push(')');
        //     }
        // }
        // tmp.push_str(&print_node(node));
        // tmp
    }

    /// Returns full tree in newick format; utf-8 encoded slice 
    fn to_newick(&self)->String{
        self.subtree_to_newick(self.get_root_id())
    }

    fn from_newick(&mut self, newick_string: String);
    // {
    //     let mut stack : Vec<Self::NodeID> = Vec::new();
    //     let mut context : Self::NodeID = 0;
    //     let mut taxa_str = String::new();
    //     let mut decimal_str: String = String::new();
    //     let mut str_ptr: usize = 0;
    //     let newick_string = newick_string.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>();
    //     while str_ptr<newick_string.len(){
    //         match newick_string[str_ptr]{
    //             '(' => {
    //                 stack.push(context);
    //                 context = self.add_node();
    //                 str_ptr +=1;
    //             },
    //             ')'|',' => {
    //                 // last context id
    //                 let last_context:&usize = stack.last().expect("Newick string ended abruptly!");
    //                 // add current context as a child to last context
    //                 self.set_child(
    //                     &context,
    //                     last_context,
    //                     decimal_str.parse::<f64>().ok(),
    //                     match taxa_str.is_empty(){
    //                         true => None,
    //                         false => Some(taxa_str.to_string())
    //                     }
    //                 );
    //                 // we clear the strings
    //                 taxa_str.clear();
    //                 decimal_str.clear();

    //                 match newick_string[str_ptr] {
    //                     ',' => {
    //                         context = self.add_node();
    //                         str_ptr += 1;
    //                     }
    //                     _ => {
    //                         context = stack.pop().expect("Newick string ended abruptly!");
    //                         str_ptr += 1;
    //                     }
    //                 }
    //             },
    //             ';'=>{
    //                 if !taxa_str.is_empty(){
    //                     self.assign_taxa(&context, &taxa_str);
    //                 }
    //                 break;
    //             }
    //             ':' => {
    //                 // if the current context had a weight
    //                 if newick_string[str_ptr]==':'{
    //                     str_ptr+=1;
    //                     while newick_string[str_ptr].is_ascii_digit() || newick_string[str_ptr]=='.'{
    //                         decimal_str.push(newick_string[str_ptr]); 
    //                         str_ptr+=1;
    //                     }
    //                 }
    //             }
    //             _ => {
    //                 // push taxa characters into taxa string
    //                 while newick_string[str_ptr]!=':'&&newick_string[str_ptr]!=')'&&newick_string[str_ptr]!=','&&newick_string[str_ptr]!='('&&newick_string[str_ptr]!=';'{
    //                     taxa_str.push(newick_string[str_ptr]); 
    //                     str_ptr+=1;
    //                 }
    //             },
    //         }
    //     }
    //     let mut leaf_ids = Vec::new();
    //     self.leaves_of_node(self.get_root(), &mut leaf_ids);
    //     for leaf_id in leaf_ids{
    //         self.set_leaf(&leaf_id);
    //     }
    // }

    fn leaves_of_node(&self, node_id:Rc<Self::NodeID>, leaves:&mut Vec<Rc<Self::NodeID>>){
        if self.get_node_children(Rc::clone(&node_id)).into_iter().collect_vec().len()==0{
            leaves.push(Rc::clone(&node_id));
        }

        for child_node_id in self.get_node_children(node_id).clone().into_iter(){
            self.leaves_of_node(child_node_id, leaves);
        }
    }
    
}

pub trait RPhyTree<RHS=Self>: SimpleRTree
{
    /// SPR function
    fn spr(&mut self, edge1: (&<Self as SimpleRTree>::NodeID, &<Self as SimpleRTree>::NodeID), edge2: (&<Self as SimpleRTree>::NodeID, &<Self as SimpleRTree>::NodeID), edge2_weights: (Option<<Self as SimpleRTree>::EdgeWeight>, Option<<Self as SimpleRTree>::EdgeWeight>)){
        let graft_edge_weight = self.get_edge_weight(edge1.0, edge1.1).cloned();
        let pruned_tree = self.prune(edge1.1);
        self.graft(pruned_tree, edge2, edge2_weights, None);
    }

    /// Balance a binary tree of 4 taxa
    fn balance_subtree(&mut self){
        assert!(self.get_leaves(self.get_root()).len()==4, "Quartets have 4 leaves!");
        assert!(!self.is_weighted(), "Cannot balance weighted tree!");
        assert!(self.is_binary(), "Cannot balance non-binary tree!");
        let (root_children, root) = (self.get_node_children(self.get_root()), *self.get_root());
        let (child1, child2) = (root_children[0], root_children[1]);
        match ((self.is_leaf(&child1)), (self.is_leaf(&child2))){
            (false, false) => {},
            (true, false) => {
                let other_leaf = &self.get_node_children(&child2).into_iter().filter(|id| self.is_leaf(id)).collect_vec()[0].clone();
                self.spr((&root, &child1), (&child2, &other_leaf), (None, None));
            },
            (false, true) => {
                let other_leaf = &self.get_node_children(&child1).into_iter().filter(|id| self.is_leaf(id)).collect_vec()[0].clone();
                self.spr((&root, &child2), (&child1, &other_leaf), (None, None));
            },
            _ =>{}
        }
        self.clean()
    }

}
