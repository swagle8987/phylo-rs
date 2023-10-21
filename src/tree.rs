pub mod simple_rtree;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::node::*;
use crate::tree::simple_rtree::*;
use crate::iter::{node_iter::*, edge_iter::*};

pub struct UnrootedPhyloTree{
    nodes: HashMap<NodeID, NodeType>,
    neighbours: HashMap<NodeID, HashSet<(Option<EdgeWeight>, NodeID)>>,
    leaves: HashMap<NodeID, String>,
}

#[derive(Debug)]
pub struct RootedPhyloTree{
    root: NodeID,
    nodes: HashMap<NodeID, NodeType>,
    children: HashMap<NodeID, Vec<(NodeID, Option<EdgeWeight>)>>,
    parents: HashMap<NodeID, Option<NodeID>>,
    leaves: HashMap<NodeID, String>,
}

impl RootedPhyloTree{
    pub fn new()->Self{
        RootedPhyloTree { 
            root: 0,
            nodes: HashMap::from([(0, false)]),
            children: HashMap::from([(0, Vec::new())]),
            parents: HashMap::from([(0, None)]),
            leaves: HashMap::new()
        }
    }

    pub fn from_newick(newick_string: String)->Self{
        let mut tree = RootedPhyloTree::new();
        let mut stack : Vec<NodeID> = Vec::new();
        let mut context : NodeID = tree.get_root().clone();
        let mut taxa_str = String::new();
        let mut decimal_str: String = String::new();
        let mut str_ptr: usize = 0;
        let newick_string = newick_string.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>();
        while str_ptr<newick_string.len(){
            dbg!(&stack, &context, &taxa_str, &decimal_str);
            match dbg!(newick_string[str_ptr]){
                '(' => {
                    stack.push(context);
                    context = tree.add_node();
                    str_ptr +=1;
                },
                ')'|',' => {
                    // last context id
                    let last_context = stack.last().expect("Newick string ended abruptly!");
                    // add current context as a child to last context
                    tree.set_child(&context, last_context, decimal_str.parse::<EdgeWeight>().ok(), taxa_str.clone());
                    // we clear the strings
                    taxa_str.clear();
                    decimal_str.clear();

                    if newick_string[str_ptr]==','{
                        // create next child of last context
                        context = tree.add_node();
                        str_ptr += 1;
                    }
                    else{
                        context = stack.pop().expect("Newick string ended abruptly!");
                        str_ptr += 1;
                    }
                },
                ';'=>{
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
                    while newick_string[str_ptr]!=':'&&newick_string[str_ptr]!=')'&&newick_string[str_ptr]!=','&&newick_string[str_ptr]!='('{
                        taxa_str.push(dbg!(newick_string[str_ptr])); 
                        str_ptr+=1;
                    }
                },
            }
        }
        return tree;
    }

    fn leaves_of_node(&self, node_id:&NodeID, leaves:&mut Vec<NodeID>){
        if self.get_node_children(node_id).is_empty(){
            leaves.push(*node_id);
        }

        for (child_node_id, _edge_weight) in self.get_node_children(node_id).iter(){
            self.leaves_of_node(child_node_id, leaves);
        }
    }

    pub fn iter_node_ancestors_pre(&self, node_id:&NodeID)->Vec<NodeID>{
        let mut node_iter: Vec<NodeID> = Vec::new();
        let mut curr_node = node_id;
        while self.parents.get(curr_node) != None {
            match self.parents.get(curr_node).expect("Invalid NodeID!") {
                Some(node) => {
                    node_iter.push(node.clone());
                    curr_node = node;
                },
                None => {
                    node_iter.push(self.get_root().clone());
                    break;
                },
            }
        }
        node_iter
    }

    fn remove_self_loops(&mut self){
        for (node_id, children) in self.children.iter_mut(){
            children.retain(|(child_id, _edge_weight)| child_id!=node_id);
        }
    }
}

impl SimpleRTree for RootedPhyloTree{
    fn add_node(&mut self)->NodeID{
        // New node id
        let node_id = self.nodes.len();
        // add entry of node in parents and children fields
        self.nodes.insert(node_id.clone(), false);
        self.parents.insert(node_id.clone(), None);
        self.children.insert(node_id.clone(), Vec::new());
        node_id
    }

    fn set_child(&mut self, node_id:&NodeID, parent_id:&NodeID, distance:Option<EdgeWeight>, taxa:String){
        self.parents.insert(node_id.clone(), Some(parent_id.clone()));
        self.children.entry(parent_id.clone()).or_default().push((node_id.clone(), distance));
        if taxa.len()>0{
            self.leaves.insert(node_id.clone(), taxa);
            self.nodes.insert(node_id.clone(), true);
        }
    }

    fn assign_taxa(&mut self,node:&NodeID, taxa:&str) {
        *self.leaves.entry(*node).or_insert(String::new()) = String::from(taxa);
    }

    fn set_edge_weight(&mut self, parent:&NodeID, child:&NodeID, edge_weight:Option<EdgeWeight>){
        self.children.entry(parent.clone())
            .and_modify(|children| *children = children.clone().iter()
                    .map(|(id, w)| {
                        match id==child{
                            true => {(id.clone(), edge_weight)},
                            false => {(id.clone(), w.clone())},
                        }
                    })
                    .collect()
        );
    }

    fn get_root(&self)->&NodeID{
        &self.root
    }

    fn get_nodes(&self)->&HashMap<NodeID, NodeType>{
        &self.nodes
    }

    fn get_node_children(&self, node_id: &NodeID)->&Vec<(NodeID, Option<EdgeWeight>)>{
        self.children.get(node_id).expect("Invalid NodeID!")
    }

    fn get_node_parent(&self, node_id:&NodeID)->Option<&NodeID>{
        self.parents.get(node_id).expect("Invalid NodeID!").as_ref()
    }

    fn get_leaves(&self, node_id: &NodeID)->HashSet<NodeID>{
        let mut leaf_vec: Vec<NodeID> = Vec::new();
        self.leaves_of_node(node_id, &mut leaf_vec);
        leaf_vec.into_iter().collect::<HashSet<NodeID>>()
    }

    fn get_subtree(&self, node_id: &NodeID)->Box<dyn SimpleRTree>{
        if self.is_leaf(node_id){
            panic!("NodeID is a leaf");
        }
        let root= node_id.clone();
        let mut nodes: HashMap<NodeID, NodeType>= HashMap::new();
        let mut children: HashMap<NodeID, Vec<(NodeID, Option<EdgeWeight>)>> = HashMap::new();
        let mut parents: HashMap<NodeID, Option<NodeID>> = HashMap::new();
        let mut leaves: HashMap<NodeID, String> = HashMap::new();
        for decsendant_node_id in self.iter_node_pre(node_id){
            nodes.insert(decsendant_node_id.clone(), self.nodes.get(&decsendant_node_id).expect("Invalid NodeID!").clone());
            children.insert(decsendant_node_id.clone(), self.children.get(&decsendant_node_id).expect("Invalid NodeID!").clone());
            parents.insert(decsendant_node_id.clone(), self.parents.get(&decsendant_node_id).expect("Invalid NodeID!").clone());
            if self.is_leaf(&decsendant_node_id){
                leaves.insert(decsendant_node_id.clone(), self.leaves.get(&decsendant_node_id).cloned().unwrap());
            }
        }
        Box::new(
            RootedPhyloTree{
                root: root,
                nodes: nodes,
                children: children,
                parents: parents,
                leaves: leaves,
            }
        )
    }

    fn get_mrca(&self, node_id_list: Vec<&NodeID>)->NodeID{
        let ancestor_iter_vec: Vec<std::vec::IntoIter<NodeID>> = node_id_list.iter().map(|x| self.iter_node_ancestors_pre(x).into_iter()).collect();
        let mut mrca: NodeID = 0;
        for mut iterator in ancestor_iter_vec{
            let temp: HashSet<NodeID> = HashSet::new();
            match iterator.next(){
                Some(x) => {
                    match temp.contains(&x){
                        true => {mrca = x.clone()},
                        false => {
                            match temp.len()==0{
                                true => {},
                                false => {return mrca}
                            }
                        }
                    }
                },
                None => {}
            }
        }
        mrca
    }

    fn is_leaf(&self, node_id: &NodeID)->bool{
        self.nodes.get(node_id).expect("Invalid NodeID").clone()
    }

    fn graft_subtree(&mut self, tree: Box<dyn SimpleRTree>, edge: (&NodeID, &NodeID)){
        todo!()
    }

    fn extract_subtree(&mut self, node_id: &NodeID)-> Box<dyn SimpleRTree>{
        let root= node_id.clone();
        let mut nodes: HashMap<NodeID, NodeType>= HashMap::new();
        let mut children: HashMap<NodeID, Vec<(NodeID, Option<EdgeWeight>)>> = HashMap::new();
        let mut parents: HashMap<NodeID, Option<NodeID>> = HashMap::new();
        let mut leaves: HashMap<NodeID, String> = HashMap::new();
        for decsendant_node_id in self.iter_node_pre(node_id){
            nodes.insert(decsendant_node_id.clone(), self.nodes.remove(&decsendant_node_id).expect("Invalid NodeID!").clone());
            children.insert(decsendant_node_id.clone(), self.children.remove(&decsendant_node_id).expect("Invalid NodeID!").clone());
            parents.insert(decsendant_node_id.clone(), self.parents.remove(&decsendant_node_id).expect("Invalid NodeID!").clone());
            match self.leaves.remove(&decsendant_node_id){
                Some(taxa_id) => {leaves.insert(decsendant_node_id.clone(), taxa_id);},
                None => {},
            }
        }
        Box::new(
            RootedPhyloTree{
                root: root,
                nodes: nodes,
                children: children,
                parents: parents,
                leaves: leaves,
            }
        )
    }

    fn iter_node_pre(&self, start_node_id: &NodeID)->PreOrdNodes{
        PreOrdNodes::new(start_node_id, &self.children)
    }

    fn iter_node_post(&self, start_node_id: &NodeID)->PostOrdNodes{
        PostOrdNodes::new(start_node_id, &self.children)
    }

    fn iter_edges_pre(&self, start_node_id: &NodeID)->PreOrdEdges{
        todo!()
    }

    fn iter_edges_post(&self, start_node_id: &NodeID)->PostOrdEdges{
        todo!()
    }

    fn get_ancestors(&self, node_id: &NodeID)->Vec<&NodeID>{
        todo!()
    }

    fn leaf_distance_matrix(&self, weighted: bool)->Vec<Vec<EdgeWeight>>{
        todo!()
    }

    fn reroot_at_node(&mut self, node_id: &NodeID){
        todo!()
    }

    fn reroot_at_edge(&mut self, edge: (&NodeID, &NodeID)) {
        todo!()
    }

    fn insert_internal_node(&mut self, edge: (NodeID, NodeID), edge_weights:(Option<EdgeWeight>, Option<EdgeWeight>)){
        todo!()
    }

    fn distance_from_root(&self, weighted: bool)->f64{
        todo!()
    }

    fn get_bipartition(&self, edge: (&NodeID, &NodeID))->(HashSet<NodeID>, HashSet<NodeID>){
        let c2 = self.get_cluster(edge.1);
        (self.leaves.keys().map(|x| x.clone()).collect::<HashSet<NodeID>>().difference(&c2).map(|x| x.clone()).collect(), c2)
    }

    fn get_cluster(&self, node_id: &NodeID)-> HashSet<NodeID>{
        let mut leaves: Vec<NodeID> = Vec::new();
        self.leaves_of_node(node_id, &mut leaves);
        HashSet::from_iter(leaves)
    }

    fn clean(&mut self) {
        let mut remove_list: Vec<&NodeID> = Vec::new();
        for (node_id, is_leaf) in self.nodes.clone().iter(){
            // remove root with only one child
            if node_id==self.get_root() && self.get_node_degree(node_id)<2{
                let new_root = self.get_node_children(self.get_root())[0].0;
                self.root = new_root;
                self.parents.entry(new_root).and_modify(|x| *x = None);
                remove_list.push(node_id);
            }
            // remove nodes with only one child
            else if !is_leaf &&  self.get_node_degree(node_id)<3{
                let parent = self.get_node_parent(node_id).cloned();
                let children = self.get_node_children(node_id).clone();
                for (child_id, _edge_weight) in children.clone().into_iter(){
                    self.parents.entry(child_id.clone()).and_modify(|x| *x = parent);
                }
                self.set_children(parent.as_ref().unwrap(), &children);
            }
        }
    }
}