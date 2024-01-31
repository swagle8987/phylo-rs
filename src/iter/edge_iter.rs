// use crate::node::*;
// use crate::tree::simple_rtree::*;
// use std::collections::HashMap;
// use crate::iter::node_iter::{PostOrdNodes, PreOrdNodes};
// use crate::tree::RootedPhyloTree;

// pub struct PreOrdEdges
// {
//     node_iter: PreOrdNodes,
//     parents: HashMap<NodeID, (Option<NodeID>, Option<EdgeWeight>)>,
// }

// impl PreOrdEdges
// {
//     pub fn new(tree: &RootedPhyloTree, start_node: &NodeID)->Self{
//         Self { 
//             node_iter: PreOrdNodes::new(
//                 start_node,
//                 tree.get_children(),
//             ),
//             parents: tree.get_parents().into_iter()
//             .filter(|(_child_id, parent_id)| parent_id!=&&None)
//             .map(|(child_id, parent_id)| (child_id.clone(), (parent_id.clone(), tree.get_edge_weight(parent_id.as_ref().unwrap(), child_id).cloned()))).collect(),
//         }
//     }
// }

// impl Iterator for PreOrdEdges
// {
//     type Item = (NodeID, NodeID, Option<EdgeWeight>);

//     fn next(&mut self)->Option<Self::Item>{
//         while let Some(next_node) = self.node_iter.next() {
//             match next_node {
//                 0 => {
//                     continue;
//                 }
//                 _ => {
//                     let parents = self.parents.get(&next_node).unwrap();
//                     return Some((parents.0.unwrap(), next_node, parents.1));
//                 }
//             }
//         }
//         None
//     }
// }

// pub struct PostOrdEdges
// {
//     stack: Vec<(NodeID, NodeID, Option<EdgeWeight>)>,
//     node_iter: PostOrdNodes,
//     children: HashMap<NodeID, Vec<(NodeID, Option<EdgeWeight>)>>,
//     parents: HashMap<NodeID, Option<NodeID>>,
// }

// impl PostOrdEdges
// {
//     pub fn new(tree: &RootedPhyloTree, start_node: &NodeID)->Self{
//         Self { 
//             stack:vec![], 
//             node_iter: PostOrdNodes::new(
//                 start_node,
//                 tree.get_children(),
//             ),
//             children: tree.get_children().clone(),
//             parents: tree.get_parents().clone(),
//         }
//     }
// }

// impl Iterator for PostOrdEdges
// {
//     type Item = (NodeID, NodeID, Option<EdgeWeight>);

//     fn next(&mut self)->Option<Self::Item>{
//         match self.stack.pop(){
//             Some((n1, n2, w)) => Some((n1, n2, w)),
//             None => {
//                 match self.node_iter.next(){
//                     Some(node_id) => {
//                         let node_id_parent = self.parents.get(&node_id).unwrap();
//                         match node_id_parent {
//                             Some(parent_id) => {
//                                 let mut w: Option<EdgeWeight> = None;
//                                 for (child_node_id, weight) in self.children.get(parent_id).unwrap(){
//                                     if child_node_id==&node_id{
//                                         w = *weight;
//                                     }
//                                 }
//                                 Some((*parent_id, node_id, w))
//                             },
//                             None => None,
//                         }
//                     },
//                     None => None
//                 }
//             }
//         }    }
// }