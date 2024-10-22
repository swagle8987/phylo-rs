#![allow(clippy::needless_lifetimes)]

/// Module with traits of rooted tree nodes
pub mod simple_rnode;

use crate::node::simple_rnode::{
    RootedMetaNode, RootedTreeNode, RootedWeightedNode, RootedZetaNode,
};
use std::fmt::{Debug, Display};

/// A type alias for NodeID
pub type NodeID = usize;

/// A node structure in an arena-memory managed tree, linking to connected neighbours via NodeID
#[derive(Clone)]
pub struct Node {
    /// A unique identifier for a node
    id: NodeID,
    /// A link to the node parent (set to None for root)
    parent: Option<NodeID>,
    /// Children of node
    children: Vec<NodeID>,
    /// Taxa annotation of node
    taxa: Option<String>,
    /// Weight of edge ending in node
    weight: Option<f32>,
    /// Real number annotation of node (used by some algorithms)
    zeta: Option<f32>,
}

impl RootedTreeNode for Node {
    type NodeID = NodeID;

    fn new(id: Self::NodeID) -> Self {
        Node {
            id,
            parent: None,
            children: vec![],
            taxa: None,
            weight: None,
            zeta: None,
        }
    }

    fn get_id(&self) -> Self::NodeID {
        self.id
    }

    fn set_id(&mut self, id: Self::NodeID) {
        self.id = id
    }

    fn set_parent(&mut self, parent: Option<Self::NodeID>) {
        self.parent = parent;
    }

    fn get_parent(&self) -> Option<Self::NodeID> {
        self.parent
    }

    fn get_children(&self) -> impl ExactSizeIterator<Item = Self::NodeID> + DoubleEndedIterator {
        self.children.clone().into_iter()
    }

    fn add_child(&mut self, child: Self::NodeID) {
        self.children.push(child);
    }

    fn remove_child(&mut self, child: &Self::NodeID) {
        self.children.retain(|x| x != child);
    }
}

impl RootedMetaNode for Node {
    type Meta = String;

    fn get_taxa<'a>(&'a self) -> Option<&'a Self::Meta> {
        self.taxa.as_ref()
    }

    fn set_taxa(&mut self, taxa: Option<Self::Meta>) {
        self.taxa = taxa;
    }
}

impl RootedWeightedNode for Node {
    type Weight = f32;

    fn get_weight(&self) -> Option<Self::Weight> {
        self.weight
    }

    fn set_weight(&mut self, w: Option<Self::Weight>) {
        self.weight = w;
    }
}

impl RootedZetaNode for Node {
    type Zeta = f32;

    fn get_zeta(&self) -> Option<Self::Zeta> {
        self.zeta
    }

    fn set_zeta(&mut self, w: Option<Self::Zeta>) {
        self.zeta = w;
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}",
            self.get_id(),
            self.node_type(),
            match self.get_taxa() {
                None => "No Taxa".to_string(),
                Some(t) => t.to_string(),
            },
            match self.get_weight() {
                None => "Unweighted".to_string(),
                Some(t) => t.to_string(),
            },
            match self.get_zeta() {
                None => "No Zeta".to_string(),
                Some(z) => z.to_string(),
            }
        )
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}",
            self.get_id(),
            self.node_type(),
            match self.get_taxa() {
                None => "None".to_string(),
                Some(t) => t.to_string(),
            },
            match self.get_weight() {
                None => "".to_string(),
                Some(t) => t.to_string(),
            },
            match self.get_zeta() {
                None => "No Zeta".to_string(),
                Some(z) => z.to_string(),
            }
        )
    }
}
