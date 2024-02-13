use crate::iter::node_iter::Ancestors;
use super::{weighted::WeightedTree, Clusters, RootedTree, DFS};

pub trait SPR
where
    Self: DFS + Sized
{
    /// Attaches input tree to self by spliting an edge
    fn graft(&mut self, tree: Self, edge: (Self::NodeID, Self::NodeID));

    /// Returns subtree starting at given node, while corresponding nodes from self.
    fn prune(&mut self, node_id: Self::NodeID)-> Self;

    /// SPR function
    fn spr(&mut self, edge1: (Self::NodeID, Self::NodeID), edge2: (Self::NodeID, Self::NodeID))
    {
        let pruned_tree = SPR::prune(self, edge1.1);
        SPR::graft(self, pruned_tree, edge2);
    }
}

pub trait NNI
where
    Self: RootedTree + Sized
{
    fn nni(&mut self, parent_id: Self::NodeID);
}

pub trait Reroot
where
    Self: RootedTree + Sized
{
    fn reroot_at_node(&mut self, node_id: Self::NodeID);
    fn reroot_at_edge(&mut self, edge: (Self::NodeID, Self::NodeID));
}

pub trait Balance
where
    Self: Clusters + SPR + Sized
{
    fn balance_subtree(&mut self);
}

pub trait Subtree
where
    Self: Ancestors + DFS + Sized
{
    fn induce_tree(&self, node_id_list: impl IntoIterator<Item=Self::NodeID, IntoIter = impl ExactSizeIterator<Item = Self::NodeID>>)->Self
    {
        let new_root_id = self.get_root_id();
        let mut subtree: Self = RootedTree::new(new_root_id);
        for node_id in node_id_list.into_iter()
        {
            let ancestors = self.ancestors(node_id);
            subtree.set_nodes(ancestors);
        }
        subtree.clean();
        subtree
    }
    fn subtree(&self, node_id: Self::NodeID)->Self
    {
        let new_root_id = self.get_root_id();
        let mut subtree: Self = RootedTree::new(new_root_id);
        let dfs = self.dfs(node_id);
        subtree.set_nodes(dfs);
        subtree

    }
}

pub trait RobinsonFoulds
where
    Self: RootedTree + Sized
{
    fn rfs(&self, tree: Self)->usize;
}

pub trait ClusterAffinity
where
    Self: RootedTree + Sized
{
    fn ca(&self, tree: Self)->usize;
}

pub trait WeightedRobinsonFoulds
where
    Self: WeightedTree + Sized
{
    fn wrfs(&self, tree: Self)->usize;
}