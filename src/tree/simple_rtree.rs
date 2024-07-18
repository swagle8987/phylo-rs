use crate::node::simple_rnode::*;
use itertools::Itertools;
use std::fmt::Debug;

pub type TreeNodeID<'a, T> = <<T as RootedTree<'a>>::Node as RootedTreeNode>::NodeID;
pub type TreeNodeMeta<'a, T> = <<T as RootedTree<'a>>::Node as RootedMetaNode<'a>>::Meta;
pub type TreeNodeWeight<'a, T> = <<T as RootedTree<'a>>::Node as RootedWeightedNode>::Weight;

/// A trait describing the behaviour of a rooted tree
pub trait RootedTree<'a>: Clone + Sync
where
    Self::Node: 'a + RootedTreeNode + Debug,
{
    type Node: 'a;

    /// Returns reference to node by ID
    fn get_node(&'a self, node_id: TreeNodeID<'a, Self>) -> Option<&'a Self::Node>;

    fn get_node_mut(&'a mut self, node_id: TreeNodeID<'a, Self>) -> Option<&'a mut Self::Node>;

    fn get_node_ids(&self) -> impl Iterator<Item = TreeNodeID<'a, Self>>;

    fn get_nodes(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Node>;

    fn get_nodes_mut(&'a mut self) -> impl Iterator<Item = &'a mut Self::Node>;

    fn get_root_id(&self) -> TreeNodeID<'a, Self>;

    fn set_root(&'a mut self, node_id: TreeNodeID<'a, Self>);

    /// Returns reference to node by ID
    fn set_node(&'a mut self, node: Self::Node);

    fn add_child(&'a mut self, parent_id: TreeNodeID<'a, Self>, child: Self::Node);

    fn remove_node(&'a mut self, node_id: TreeNodeID<'a, Self>) -> Option<Self::Node>;

    fn delete_node(&'a mut self, node_id: TreeNodeID<'a, Self>);

    fn contains_node(&self, node_id: TreeNodeID<'a, Self>) -> bool;

    fn clean(&'a mut self);

    fn clear(&'a mut self);

    fn delete_edge(&'a mut self, parent_id: TreeNodeID<'a, Self>, child_id: TreeNodeID<'a, Self>);

    fn set_nodes(
        &'a mut self,
        node_list: impl IntoIterator<
            Item = Self::Node,
            IntoIter = impl ExactSizeIterator<Item = Self::Node>,
        >,
    );

    fn split_edge(
        &'a mut self,
        edge: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>),
        node: Self::Node,
    );

    fn add_sibling(
        &'a mut self,
        node_id: TreeNodeID<'a, Self>,
        split_node: Self::Node,
        sibling_node: Self::Node,
    );

    fn get_leaves(&'a self) -> impl ExactSizeIterator<Item = &'a Self::Node> {
        self.get_nodes()
            .filter(|x| x.is_leaf())
            .collect_vec()
            .into_iter()
    }

    fn get_leaf_ids(&self) -> impl ExactSizeIterator<Item = TreeNodeID<'a, Self>> {
        self.get_node_ids()
            .filter(|x| self.is_leaf(*x))
            .collect_vec()
            .into_iter()
    }

    /// Get root node
    fn get_root(&'a self) -> &'a Self::Node {
        self.get_node(self.get_root_id()).unwrap()
    }

    fn get_root_mut(&'a mut self) -> &'a mut Self::Node;

    fn set_child(&'a mut self, parent_id: TreeNodeID<'a, Self>, child_id: TreeNodeID<'a, Self>);

    fn remove_child(&'a mut self, parent_id: TreeNodeID<'a, Self>, child_id: TreeNodeID<'a, Self>) {
        self.get_node_mut(parent_id)
            .unwrap()
            .remove_child(&child_id);
    }

    fn remove_children(
        &'a mut self,
        parent_id: TreeNodeID<'a, Self>,
        child_ids: Vec<TreeNodeID<'a, Self>>,
    );

    fn remove_all_children(&'a mut self, node_id: TreeNodeID<'a, Self>);

    fn get_node_parent_id(&self, node_id: TreeNodeID<'a, Self>) -> Option<TreeNodeID<'a, Self>>;

    fn get_node_parent(&'a self, node_id: TreeNodeID<'a, Self>) -> Option<&'a Self::Node> {
        match self.get_node_parent_id(node_id) {
            Some(id) => self.get_node(id),
            None => None,
        }
    }

    fn get_node_children(
        &'a self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl ExactSizeIterator<Item = &'a Self::Node> {
        let node = self.get_node(node_id).unwrap();
        node.get_children().map(|x| self.get_node(x).unwrap())
    }

    fn get_node_children_ids(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl ExactSizeIterator<Item = TreeNodeID<'a, Self>>;

    fn node_degree(&'a self, node_id: TreeNodeID<'a, Self>) -> usize {
        self.get_node(node_id).unwrap().degree()
    }
    fn get_node_depth(&self, node_id: TreeNodeID<'a, Self>) -> usize {
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

    fn is_leaf(&self, node_id: TreeNodeID<'a, Self>) -> bool;

    fn num_nodes(&'a self) -> usize {
        self.get_nodes().len()
    }

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

    fn get_sibling_ids(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> impl Iterator<Item = TreeNodeID<'a, Self>> {
        let parent_id = self.get_node_parent_id(node_id).expect("Root does not have siblings!");
        let sibling_ids = self.get_node_children_ids(parent_id).filter(move |x| x != &node_id);
        return sibling_ids;
    }

    fn supress_node(&'a mut self, _node_id: TreeNodeID<'a, Self>) {
        todo!()
    }

    fn supress_unifurcations(&'a mut self) {
        todo!()
    }
}

pub trait RootedMetaTree<'a>: RootedTree<'a>
where
    Self::Node: RootedMetaNode<'a>,
{
    fn get_taxa_node(&'a self, taxa: &TreeNodeMeta<'a, Self>) -> Option<&'a Self::Node>;

    fn get_taxa_node_id(&self, taxa: &TreeNodeMeta<'a, Self>) -> Option<TreeNodeID<'a, Self>>;

    fn num_taxa(&self) -> usize;

    fn set_node_taxa(
        &'a mut self,
        node_id: TreeNodeID<'a, Self>,
        taxa: Option<TreeNodeMeta<'a, Self>>,
    ) {
        self.get_node_mut(node_id).unwrap().set_taxa(taxa)
    }
    
    fn get_node_taxa(
        &'a self,
        node_id: TreeNodeID<'a, Self>,
    ) -> Option<&'a TreeNodeMeta<'a, Self>> {
        self.get_node(node_id).unwrap().get_taxa()
    }

    fn get_node_taxa_cloned(
        &self,
        node_id: TreeNodeID<'a, Self>,
    ) -> Option<TreeNodeMeta<'a, Self>>;

    fn get_taxa_space(&'a self) -> impl ExactSizeIterator<Item = &'a TreeNodeMeta<'a, Self>>;
}

pub trait RootedWeightedTree<'a>: RootedTree<'a>
where
    Self::Node: RootedWeightedNode,
{
    fn unweight(&'a mut self);

    fn set_edge_weight(
        &'a mut self,
        edge: (TreeNodeID<'a, Self>, TreeNodeID<'a, Self>),
        edge_weight: Option<TreeNodeWeight<'a, Self>>,
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
        _parent_id: TreeNodeID<'a, Self>,
        child_id: TreeNodeID<'a, Self>,
    ) -> Option<TreeNodeWeight<'a, Self>> {
        self.get_node(child_id).unwrap().get_weight()
    }
}
