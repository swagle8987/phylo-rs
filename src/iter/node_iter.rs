use crate::node::simple_rt_node::*;
use crate::tree::simple_rtree::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::{Display, Debug};
use itertools::Itertools;


pub struct PreOrdNodes<T>
{
    stack: Vec<NodeID>,
    nodes: HashMap<NodeID, HashMap<T, NodeID>>
}

impl<T> PreOrdNodes<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone
{
    pub fn new(start_node_id: &NodeID, _tree: &HashMap<NodeID, (EdgeWeight, NodeID)>)->Self{
        Self { stack:vec![*start_node_id], nodes: HashMap::new()}
    }
}

impl<T> Iterator for PreOrdNodes<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone
{
    type Item = NodeID;

    fn next(&mut self)->Option<Self::Item>{
        match self.stack.pop() {
            Some(node_id) => {
                let children_ids:Vec<&NodeID> = self.nodes.get(&node_id).expect("Invalid Node ID!").values().collect();
                for child_node_id in children_ids.into_iter().sorted(){
                    self.stack.push(*child_node_id)
            }
            Some(node_id)
            }
            None => None,
        }
    }
}

pub struct PostOrdNodes<T>
{
    stack: Vec<NodeID>,
    nodes: HashMap<NodeID, HashMap<T, NodeID>>
}

impl<T> PostOrdNodes<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone
{
    pub fn new(start_node_id: &NodeID, _tree: &HashMap<NodeID, (EdgeWeight, NodeID)>)->Self{
        Self { stack:vec![*start_node_id], nodes: HashMap::new()}
    }
}

impl<T> Iterator for PostOrdNodes<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone
{
    type Item = NodeID;

    fn next(&mut self)->Option<Self::Item>{
        while let Some(node_id) = self.stack.pop()  {
            if self.nodes.contains_key(&node_id){
                self.stack.push(node_id);
                let children = self.nodes.remove(&node_id).unwrap();
                for child_id in children.values().sorted(){
                    self.stack.push(*child_id)
                }
            }
            else{
                return Some(node_id)
            }
        }
        None
    }
}