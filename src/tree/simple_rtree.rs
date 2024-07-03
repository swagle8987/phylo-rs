use crate::node::simple_rnode::*;
use itertools::Itertools;
use std::fmt::Debug;

/// A trait describing the behaviour of a rooted tree
pub trait RootedTree<'a>: Clone + Sync
where
    Self::Node: 'a + RootedTreeNode + Debug, // + AsRef<Self::Node>,
{
    type Node: 'a;

    /// Returns reference to node by ID
    fn get_node(&'a self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> Option<&'a Self::Node>;

    fn get_node_mut(&'a mut self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> Option<&'a mut Self::Node>;

    fn get_node_ids(&self) -> impl Iterator<Item = <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID>;

    fn get_nodes(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Node>;

    fn get_nodes_mut(&'a mut self) -> impl Iterator<Item = &'a mut Self::Node>;

    fn get_root_id(&'a self) -> <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID;

    fn set_root(&'a mut self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID);

    /// Returns reference to node by ID
    fn set_node(&'a mut self, node: Self::Node);

    fn add_child(&'a mut self, parent_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, child: Self::Node);

    fn remove_node(&'a mut self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> Option<Self::Node>;

    fn delete_node(&'a mut self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID);

    fn contains_node(&self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> bool;

    fn clean(&'a mut self);

    fn clear(&'a mut self);

    fn delete_edge(&'a mut self, parent_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, child_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID);

    fn set_nodes(
        &'a mut self,
        node_list: impl IntoIterator<
            Item = Self::Node,
            IntoIter = impl ExactSizeIterator<Item = Self::Node>,
        >,
    );

    fn split_edge(&'a mut self, edge: (<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID), node: Self::Node);

    fn add_sibling(
        &'a mut self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
        split_node: Self::Node,
        sibling_node: Self::Node,
    );

    fn get_leaves(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Node> {
        self.get_nodes()
            .filter(|x| x.is_leaf())
            .collect_vec()
            .into_iter()
    }

    /// Get root node
    fn get_root(&'a self) -> &'a Self::Node {
        self.get_node(self.get_root_id()).unwrap()
    }

    fn get_root_mut(&'a mut self) -> &'a mut Self::Node;

    fn set_child(&'a mut self, parent_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, child_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID);

    fn remove_child(&'a mut self, parent_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, child_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID)
    {
        self.get_node_mut(parent_id)
            .unwrap()
            .remove_child(&child_id);
    }

    fn remove_children(&'a mut self, parent_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, child_ids: Vec<<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID>);

    fn remove_all_children(&'a mut self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID);

    fn get_node_parent_id(&'a self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> Option<<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID> {
        self.get_node(node_id).unwrap().get_parent()
    }

    fn get_node_parent(&'a self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> Option<&'a Self::Node> {
        match self.get_node_parent_id(node_id) {
            Some(id) => self.get_node(id),
            None => None,
        }
    }

    fn get_node_children(
        &'a self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl ExactSizeIterator<Item = &'a Self::Node> {
        let node = self.get_node(node_id).unwrap();
        node.get_children()
            .map(|x| self.get_node(x).unwrap())
    }

    fn get_node_children_ids(
        &'a self,
        node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> impl ExactSizeIterator<Item = <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID> {
        self.get_node(node_id)
            .unwrap()
            .get_children()
            .collect_vec()
            .into_iter()
    }

    fn node_degree(&'a self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> usize {
        self.get_node(node_id).unwrap().degree()
    }
    fn get_node_depth(&'a self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> usize {
        let mut start_id = node_id;
        let mut depth = 0;
        while let Some(parent_id) = self.get_node_parent_id(start_id) {
            depth += 1;
            start_id = parent_id;
        }
        depth
    }
    fn is_binary(&'a self) -> bool {
        for node in self.get_nodes() {
            if node.get_children().collect_vec().len() > 2 {
                return false;
            }
        }
        true
    }

    fn is_leaf(&'a self, node_id: &<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> bool {
        self.get_node(*node_id).unwrap().is_leaf()
    }

    fn num_nodes(&'a self) -> usize {
        self.get_nodes().len()
    }

    fn get_siblings(&'a self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> impl Iterator<Item = &'a Self::Node> {
        let parent_id = self
            .get_node_parent_id(node_id)
            .expect("Node has no siblings!");
        return self
            .get_node_children(parent_id)
            .filter(move |x| x.get_id() != node_id);
    }

    fn get_sibling_ids(&'a self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> impl Iterator<Item = <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID> {
        return self.get_siblings(node_id).map(|x| x.get_id());
    }

    fn supress_node(&'a mut self, _node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) {
        todo!()
    }

    fn supress_unifurcations(&'a mut self) {
        todo!()
    }
}

pub trait RootedMetaTree<'a>: RootedTree<'a>
where
    Self::Node: RootedTreeNode + RootedMetaNode<'a>,
{
    fn get_taxa_node(&self, taxa: &<<Self as RootedTree<'a>>::Node as RootedMetaNode<'a>>::Meta) -> Option<&Self::Node>;

    fn get_taxa_node_id(&self, taxa: &<<Self as RootedTree<'a>>::Node as RootedMetaNode<'a>>::Meta) -> Option<<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID> {
        self.get_taxa_node(taxa).map(|node| node.get_id())
    }
    fn num_taxa(&'a self) -> usize {
        self.get_nodes().filter(|f| f.is_leaf()).collect_vec().len()
    }
    fn set_node_taxa(&'a mut self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, taxa: Option<<<Self as RootedTree<'a>>::Node as RootedMetaNode<'a>>::Meta>) {
        self.get_node_mut(node_id).unwrap().set_taxa(taxa)
    }
    fn get_node_taxa(&'a self, node_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID) -> Option<&'a <<Self as RootedTree<'a>>::Node as RootedMetaNode<'a>>::Meta> {
        self.get_node(node_id).unwrap().get_taxa()
    }
    fn get_taxa_space(&'a self) -> impl ExactSizeIterator<Item = &'a <<Self as RootedTree<'a>>::Node as RootedMetaNode<'a>>::Meta>;
}

pub trait RootedWeightedTree<'a>: RootedTree<'a>
where
    Self::Node: RootedWeightedNode,
{
    fn unweight(&'a mut self);

    fn set_edge_weight(
        &'a mut self,
        edge: (<<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID, <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID),
        edge_weight: Option<<<Self as RootedTree<'a>>::Node as RootedWeightedNode>::Weight>,
    ) {
        self.get_node_mut(edge.1).unwrap().set_weight(edge_weight);
    }

    fn is_weighted(&'a self) -> bool {
        for node_id in self.get_node_ids() {
            if self.get_node(node_id).unwrap().get_weight().is_none() {
                return false;
            }
        }
        true
    }

    fn get_edge_weight(
        &'a self,
        _parent_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
        child_id: <<Self as RootedTree<'a>>::Node as RootedTreeNode>::NodeID,
    ) -> Option<<<Self as RootedTree<'a>>::Node as RootedWeightedNode>::Weight> {
        self.get_node(child_id).unwrap().get_weight()
    }
}
