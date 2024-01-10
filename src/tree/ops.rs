use crate::tree::simple_rtree::SimpleRootedTree;
use std::rc::Rc;

pub trait SPR
where
    Self: SimpleRootedTree + Sized
{
    /// Attaches input tree to self by spliting an edge
    fn graft(&mut self, tree: Self, edge: (Rc<Self::NodeID>, Rc<Self::NodeID>), edge_weights:(Option<Self::EdgeWeight>, Option<Self::EdgeWeight>), graft_edge_weight: Option<Self::EdgeWeight>);

    /// Returns subtree starting at given node, while corresponding nodes from self.
    fn prune(&mut self, node_id: Rc<Self::NodeID>)-> Self;

    /// SPR function
    fn spr(&mut self, edge1: (Rc<<Self as SimpleRootedTree>::NodeID>, Rc<<Self as SimpleRootedTree>::NodeID>), edge2: (Rc<<Self as SimpleRootedTree>::NodeID>, Rc<<Self as SimpleRootedTree>::NodeID>), edge2_weights: (Option<<Self as SimpleRootedTree>::EdgeWeight>, Option<<Self as SimpleRootedTree>::EdgeWeight>)){
        let graft_edge_weight = self.get_edge_weight(edge1.0, edge1.1);
        let pruned_tree = SPR::prune(self, edge1.1);
        SPR::graft(self, pruned_tree, edge2, edge2_weights, None);
    }    
}

pub trait NNI
where
    Self: SimpleRootedTree + Sized
{
    fn nni(&mut self, parent_id: Self::NodeID);
}