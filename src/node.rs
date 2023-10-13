pub mod simple_rt_node;

// use std::ops::Index;
// use simple_rt_node::{SimpleRTNode, NodeID};

// #[derive(Debug)]
// pub struct RTNode
// {
//     children: Vec<NodeID>,
//     parent: Option<NodeID>,
//     edge_weight: f64,
// }

// impl SimpleRTNode for RTNode{
//     fn set_parent_id(&mut self, parent_id: NodeID){
//         self.parent = Some(parent_id);
//     }

//     fn get_parent_id(&self)->Option<&NodeID>{
//         self.parent.as_ref()
//     }

//     fn get_edge_weight(&self)-> Option<&f64>{
//         Some(&self.edge_weight)
//     }

//     fn set_edge_weight(&mut self, edge_weight:f64){
//         self.edge_weight = edge_weight;
//     }

//     fn add_child(&mut self, child_id: NodeID){
//         self.children.push(child_id)
//     }

//     fn has_children(&self)->bool{
//         !self.children.is_empty()
//     }

//     fn get_children(&self)->Box<(dyn Index<usize, Output=NodeID>)>{
//         Box::new(self.children.clone())
//     }

//     fn is_leaf(&self)->bool{
//         self.children.is_empty()
//     }

// }