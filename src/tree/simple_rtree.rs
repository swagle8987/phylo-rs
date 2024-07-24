use crate::node::simple_rnode::*;
use itertools::Itertools;
use anyhow::Result;
use std::fmt::Debug;

use super::TreeQueryErr;

/// A type alias for Tree Node ID
pub type TreeNodeID<'a, T> = <<T as RootedTree<'a>>::Node as RootedTreeNode>::NodeID;
/// A type alias for Tree Node meta annotation
pub type TreeNodeMeta<'a, T> = <<T as RootedTree<'a>>::Node as RootedMetaNode<'a>>::Meta;
/// A type alias for Tree edge weight
pub type TreeNodeWeight<'a, T> = <<T as RootedTree<'a>>::Node as RootedWeightedNode>::Weight;

/// A trait describing the behaviour of a rooted tree
pub trait RootedTree<'a>: Clone + Sync
where
    Self::Node: 'a + RootedTreeNode + Debug,
{
    /// An associated node type for a rooted tree
    type Node: 'a;

    /// Returns reference to node by ID
    fn get_node(&'a self, node_id: TreeNodeID<'a, Self>) -> Result<&'a Self::Node>;

    /// Returns a mutable reference to a node
    fn get_node_mut(&'a mut self, node_id: TreeNodeID<'a, Self>) -> Option<&'a mut Self::Node>;

    /// Reurns an iterator over all NodeID's
    fn get_node_ids(&self) -> impl Iterator<Item = TreeNodeID<'a, Self>>;

    /// Returns an iterator with immutable references to nodes
    fn get_nodes(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Node>;

    /// Returns iterator with mutable references to nodes
    fn get_nodes_mut(&'a mut self) -> impl Iterator<Item = &'a mut Self::Node>;

    /// Returns NodeID of root node
    fn get_root_id(&self) -> TreeNodeID<'a, Self>;

    /// Sets node with NodeID and root node
    fn set_root(&'a mut self, node_id: TreeNodeID<'a, Self>);

    /// Inserts a floating node into tree.
    fn set_node(&'a mut self, node: Self::Node);

    /// Adds node as child to an existing node in tree.
    fn add_child(&'a mut self, parent_id: TreeNodeID<'a, Self>, child: Self::Node);

    /// Removes node from tree while deleting any edges if they exist
    fn remove_node(&'a mut self, node_id: TreeNodeID<'a, Self>) -> Option<Self::Node>;

    /// Removes nodes from tree without deleting any edges that may exist
    fn delete_node(&'a mut self, node_id: TreeNodeID<'a, Self>);

    /// Returns true if node with node_id exists in tree
    fn contains_node(&self, node_id: TreeNodeID<'a, Self>) -> bool;

    /// Removes internal nodes of degree 2 and any floating nodes
    fn clean(&'a mut self);

    /// Removes all nodes from tree except root node
    fn clear(&'a mut self);

    /// Deletes an edge from the tree without deleting an nodes
    fn delete_edge(&'a mut self, parent_id: TreeNodeID<'a, Self>, child_id: TreeNodeID<'a, Self>);

    /// Inserts nodes into tree from iterator. Note: this will overwrite any existing node with a NodeID that already exists in tree.
    fn set_nodes(
        &mut self,
        node_list: impl IntoIterator<
            Item = Self::Node,
            IntoIter = impl ExactSizeIterator<Item = Self::Node>,
        >,
    );

    /// Splits an edge in the tree with provided node.
    fn split_edge(
        &'a mut self,
        edge: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>),
        node: Self::Node,
    );

    /// Add node as a sibling to the provided NodeID.
    fn add_sibling(
        &'a mut self,
        node_id: TreeNodeID<'a, Self>,
        split_node: Self::Node,
        sibling_node: Self::Node,
    );

    /// Returns iterator of immutable references to leaf nodes in tree.
    fn get_leaves(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Node> {
        self.get_nodes()
            .filter(|x| x.is_leaf())
            .collect_vec()
            .into_iter()
    }

    /// Returns an iterator of leaf NodeID's
    fn get_leaf_ids(&self) -> impl ExactSizeIterator<Item = TreeNodeID<'a, Self>> {
        self.get_node_ids()
            .filter(|x| self.is_leaf(*x))
            .collect_vec()
            .into_iter()
    }

    /// Returns an immutable reference to root node
    fn get_root(&'a self) -> &'a Self::Node {
        self.get_node(self.get_root_id()).unwrap()
    }

    /// Returns a mutable reference to the root node
    fn get_root_mut(&'a mut self) -> &'a mut Self::Node;

    /// creates an edge from node with parent ID to child ID. The child node must already exist in tree.
    fn set_child(&'a mut self, parent_id: TreeNodeID<'a, Self>, child_id: TreeNodeID<'a, Self>);

    /// Removes edge from prant to child without deleting either node.
    fn remove_child(&'a mut self, parent_id: TreeNodeID<'a, Self>, child_id: TreeNodeID<'a, Self>) {
        self.get_node_mut(parent_id)
            .unwrap()
            .remove_child(&child_id);
    }

    /// Removes set of children from parent node.
    fn remove_children(
        &'a mut self,
        parent_id: TreeNodeID<'a, Self>,
        child_ids: impl Iterator<Item = TreeNodeID<'a, Self>>,
    );

    /// Removes all children from parent node.
    fn remove_all_children(&'a mut self, node_id: TreeNodeID<'a, Self>);

    /// Returns parent ID of a node in tree
    fn get_node_parent_id(&self, node_id: TreeNodeID<'a, Self>) -> Option<TreeNodeID<'a, Self>>;

    /// Returns immutable reference to parent for a node
    fn get_node_parent(&'a self, node_id: TreeNodeID<'a, Self>) -> Result<&'a Self::Node> {
        self.get_node(self.get_node_parent_id(node_id).ok_or(TreeQueryErr("Node does not have a parent".to_string()))?)
    }

    /// Returns an iterator of immutable references to children of a node
    fn get_node_children(
        &'a self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl ExactSizeIterator<Item = &'a Self::Node> {
        let node = self.get_node(node_id).unwrap();
        node.get_children().map(|x| self.get_node(x).unwrap())
    }

    /// Returns an iterator of node children ids
    fn get_node_children_ids(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl ExactSizeIterator<Item = TreeNodeID<'a, Self>>;

    /// Returns degree of a node
    fn node_degree(&'a self, node_id: TreeNodeID<'a, Self>) -> usize {
        self.get_node(node_id).unwrap().degree()
    }

    /// Returns depth of node as number of edges in the path between node and root.
    fn get_node_depth(&self, node_id: TreeNodeID<'a, Self>) -> usize {
        let mut start_id = node_id;
        let mut depth = 0;
        while let Some(parent_id) = self.get_node_parent_id(start_id) {
            depth += 1;
            start_id = parent_id;
        }
        depth
    }

    /// Returns true if tree is binary
    fn is_binary(&self) -> bool {
        for node_id in self.get_node_ids(){
            if node_id==self.get_root_id(){
                if self.get_node_degree(node_id)!=2{
                    return false;
                }
            }
            else{
                if self.get_node_degree(node_id)%2!=1{
                    return false;
                }
            }
        }
        true
}

    /// Returns true if node with node_id is a leaf node
    fn is_leaf(&self, node_id: TreeNodeID<'a, Self>) -> bool;

    /// Returns total number of nodes in tree
    fn num_nodes(&self) -> usize {
        self.get_node_ids().collect_vec().len()
    }

    /// Returns node degree
    fn get_node_degree(&self, node_id: TreeNodeID<'a, Self>)->usize;

    /// Returns iterator of immutable references to siblings of a node.
    fn get_siblings(
        &'a self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl Iterator<Item = &'a Self::Node> {
        let parent_id = self
            .get_node_parent_id(node_id)
            .expect("Node has no siblings!");
        return self
            .get_node_children(parent_id)
            .filter(move |x| x.get_id() != node_id);
    }

    /// Returns iterator of NodeID's of node siblings
    fn get_sibling_ids(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl Iterator<Item = TreeNodeID<'a, Self>> {
        let parent_id = self.get_node_parent_id(node_id).expect("Root does not have siblings!");
        let sibling_ids = self.get_node_children_ids(parent_id).filter(move |x| x != &node_id);
        return sibling_ids;
    }

    /// Connects a nodes children to it's parent, then deletes all edges to the node, without deleting the node from the tree
    fn supress_node(&'a mut self, _node_id: TreeNodeID<'a, Self>) {
        todo!()
    }

    /// Supresses all nodes of degree 2
    fn supress_unifurcations(&'a mut self) {
        todo!()
    }
}

/// A trait describing the behaviour of a rooted tree where some of the nodes have a meta annotation. The terms meta and taxa are used interchangably here.
pub trait RootedMetaTree<'a>: RootedTree<'a>
where
    Self::Node: RootedMetaNode<'a>,
{
    ///  Returns an immutable reference to a node with a give meta annotation
    fn get_taxa_node(&'a self, taxa: &TreeNodeMeta<'a, Self>) -> Result<&'a Self::Node>;

    /// Returns the node id of a node with a meta annotation
    fn get_taxa_node_id(&self, taxa: &TreeNodeMeta<'a, Self>) -> Option<TreeNodeID<'a, Self>>;

    /// Returns totla number of nodes with a meta annotation
    fn num_taxa(&self) -> usize;

    /// Sets the emta annotation of a node
    fn set_node_taxa(
        &'a mut self,
        node_id: TreeNodeID<'a, Self>,
        taxa: Option<TreeNodeMeta<'a, Self>>,
    ) {
        self.get_node_mut(node_id).unwrap().set_taxa(taxa)
    }
    
    /// Returns an immutable reference to the meta annotation of a node, and None is there is no meta annotation 
    fn get_node_taxa(
        &'a self,
        node_id: TreeNodeID<'a, Self>,
    ) -> Option<&'a TreeNodeMeta<'a, Self>> {
        self.get_node(node_id).unwrap().get_taxa()
    }

    /// Returns a deep copy of the meta annotation of a node
    fn get_node_taxa_cloned(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> Option<TreeNodeMeta<'a, Self>>;

    /// Returns an iterator with immutable references to all meta annotations in a tree.
    fn get_taxa_space(&'a self) -> impl ExactSizeIterator<Item = &'a TreeNodeMeta<'a, Self>>;
}

/// A trait describing the behaviour of a rooted tree where some of the edges are weighted
pub trait RootedWeightedTree<'a>: RootedTree<'a>
where
    Self::Node: RootedWeightedNode,
{
    /// Sets all edge weights to None
    fn unweight(&'a mut self);

    /// Sets edge weight to None
    fn set_edge_weight(
        &'a mut self,
        edge: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>),
        edge_weight: Option<TreeNodeWeight<'a, Self>>,
    ) {
        self.get_node_mut(edge.1).unwrap().set_weight(edge_weight);
    }

    /// Returns true if edge weight not None
    fn is_weighted(&'a self) -> bool {
        for node_id in self.get_node_ids() {
            if self.get_node(node_id).unwrap().get_weight().is_none() {
                return false;
            }
        }
        true
    }

    /// Returns weight of edge
    fn get_edge_weight(
        &'a self,
        _parent_id: TreeNodeID<'a, Self>,
        child_id: TreeNodeID<'a, Self>,
    ) -> Option<TreeNodeWeight<'a, Self>> {
        self.get_node(child_id).unwrap().get_weight()
    }
}
