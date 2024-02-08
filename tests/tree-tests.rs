// use std::Arc::Arc;
use std::sync::Arc;


use itertools::Itertools;
use phylo::iter::node_iter::{Ancestors, Clusters, EulerTour, DFS};
use phylo::node::simple_rnode::RootedTreeNode;
use phylo::node::Node;
use phylo::tree::ops::{Balance, SPR, Subtree};
use phylo::tree::SimpleRootedTree;
use phylo::tree::simple_rtree::RootedTree;
use phylo::tree::io::*;

#[test]
fn build_small_tree() {
    let mut tree = SimpleRootedTree::new(Arc::new(1));
    let new_node = Node::new(Arc::new(2), false);
    tree.add_child(tree.get_root_id(), new_node);
    let new_node = Node::new(Arc::new(3), true);
    tree.add_child(tree.get_root_id(), new_node);
    let new_node: Node = Node::new(Arc::new(4), true);
    tree.add_child(Arc::new(2), new_node);
    let new_node: Node = Node::new(Arc::new(5), true);
    tree.add_child(Arc::new(2), new_node);
    // dbg!(&tree, tree.get_node(Arc::new(1)).unwrap().get_children().into_iter().collect_vec());
    // dbg!(&tree.get_node_depth(Arc::new(2)));
}
#[test]
fn tree_iter() {
    let mut tree = SimpleRootedTree::new(Arc::new(1));
    let new_node = Node::new(Arc::new(2), false);
    tree.add_child(tree.get_root_id(), new_node);
    let new_node = Node::new(Arc::new(3), true);
    tree.add_child(tree.get_root_id(), new_node);
    let new_node: Node = Node::new(Arc::new(4), true);
    tree.add_child(Arc::new(2), new_node);
    let new_node: Node = Node::new(Arc::new(5), true);
    tree.add_child(Arc::new(2), new_node);
    dbg!(&tree.get_node(Arc::new(1)).unwrap().get_children().into_iter().collect_vec());
    dbg!(&tree.dfs(tree.get_root_id()).into_iter().collect_vec());
    dbg!(&tree.euler_tour(tree.get_root_id()).into_iter().collect_vec());
    dbg!(&tree.dfs(tree.get_root_id()).into_iter().collect_vec());
    dbg!(&tree.ancestors(Arc::new(5)).into_iter().collect_vec());
}
#[test]
fn tree_mArca() {
    let mut tree = SimpleRootedTree::new(Arc::new(1));
    let new_node = Node::new(Arc::new(2), false);
    tree.add_child(tree.get_root_id(), new_node);
    let new_node = Node::new(Arc::new(3), true);
    tree.add_child(tree.get_root_id(), new_node);
    let new_node: Node = Node::new(Arc::new(4), true);
    tree.add_child(Arc::new(2), new_node);
    let new_node: Node = Node::new(Arc::new(5), true);
    tree.add_child(Arc::new(2), new_node);
    assert!(&tree.get_mrca(&vec![Arc::new(4), Arc::new(5), Arc::new(3)])==&Arc::new(1));
    assert!(&tree.get_mrca(&vec![Arc::new(4), Arc::new(3)])==&Arc::new(1));
    assert!(&tree.get_mrca(&vec![Arc::new(2), Arc::new(3)])==&Arc::new(1));
    assert!(&tree.get_mrca(&vec![Arc::new(4), Arc::new(5)])==&Arc::new(2));
}
#[test]
fn read_small_tree() {
    let input_str = String::from("((A,B),C);");
    let tree = SimpleRootedTree::from_newick(input_str.as_bytes());
    dbg!(&tree.euler_tour(tree.get_root_id()).into_iter().collect_vec());
    let input_str = String::from("((A:0.1,B:0.2),C:0.6);");
    let tree = SimpleRootedTree::from_newick(input_str.as_bytes());
    dbg!(&tree.euler_tour(tree.get_root_id()).into_iter().collect_vec());
    dbg!(format!("{}", &tree.to_newick()));
}
#[test]
fn tree_spr() {
    let input_str = String::from("((A,B),C);");
    let mut tree = SimpleRootedTree::from_newick(input_str.as_bytes());
    dbg!(format!("{}", &tree.to_newick()));
    dbg!(tree.get_nodes().into_iter().collect_vec());
    let p_tree = tree.prune(Arc::new(1));
    dbg!(format!("{}", &tree.to_newick()));
    dbg!(format!("{}", &p_tree.to_newick()));
    tree.graft(p_tree, (Arc::new(0),Arc::new(4)));
    tree.clean();
    dbg!(format!("{}", &tree.to_newick()));
    dbg!(&tree.get_node_parent(Arc::new(4)));
    tree.spr((Arc::new(1),Arc::new(2)), (Arc::new(5),Arc::new(4)));
    dbg!(format!("{}", &tree.to_newick()));
}
#[test]
fn tree_cluster() {
    let input_str: String = String::from("((A,B),C);");
    let tree = SimpleRootedTree::from_newick(input_str.as_bytes());
    dbg!(&tree.get_cluster(Arc::new(0)).into_iter().collect_vec());
    dbg!(&tree.get_cluster(Arc::new(1)).into_iter().collect_vec());
    let bp = tree.get_bipartition((Arc::new(0), Arc::new(1)));
    dbg!(&bp.0.into_iter().collect_vec());
    dbg!(&bp.1.into_iter().collect_vec());
}
#[test]
fn balance_tree() {
    let input_str: String = String::from("(((A,B),C),D);");
    let mut tree = SimpleRootedTree::from_newick(input_str.as_bytes());
    tree.balance_subtree();
    dbg!(format!("{}", &tree.to_newick()));
    let input_str: String = String::from("(D,(C,(A,B)));");
    let mut tree = SimpleRootedTree::from_newick(input_str.as_bytes());
    tree.balance_subtree();
    dbg!(format!("{}", &tree.to_newick()));
    let input_str: String = String::from("(D,(A,(C,B)));");
    let mut tree = SimpleRootedTree::from_newick(input_str.as_bytes());
    tree.balance_subtree();
    dbg!(format!("{}", &tree.to_newick()));
    dbg!(tree.get_nodes().into_iter().collect_vec());
    dbg!(tree.get_root_id());
}
#[test]
fn induce_tree() {
    let input_str: String = String::from("(((A,B),C),D);");
    let tree = SimpleRootedTree::from_newick(input_str.as_bytes());
    dbg!(format!("{}", &tree.to_newick()));
    let mut x = tree.induce_tree(vec![Arc::new(3), Arc::new(5), Arc::new(6)]);
    x.clean();
    dbg!(x.get_root().get_children().into_iter().collect_vec());
    dbg!(x.get_nodes().into_iter().collect_vec());
    dbg!(format!("{}", &x.to_newick()));
}
// #[test]
// fn balance_tree() {
//     let input_str = String::from("(0,(1,(2,(3,(4,(5,(6,(7,(8,(9,(10,(11,(12,(13,(14,(15,(16,(17,(18,(19,(20,(21,(22,(23,(24,(25,(26,(27,(28,(29,(30,(31,(32,(33,(34,(35,(36,(37,(38,(39,(40,(41,(42,(43,(44,(45,(46,(47,(48,(49,(50,(51,(52,(53,(54,(55,(56,(57,(58,(59,(60,(61,(62,(63,(64,(65,(66,(67,(68,(69,(70,(71,(72,(73,(74,(75,(76,(77,(78,(79,(80,(81,(82,(83,(84,(85,(86,(87,(88,(89,(90,(91,(92,(93,(94,(95,(96, (97,98))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))");
//     let tree = RootedPhyloTree::from_newick(input_str);
//     dbg!(tree.to_newick());
//     let mut x = tree.induce_tree(vec!["2".to_string(), "4".to_string(), "8".to_string(), "10".to_string()]);
//     dbg!(x.to_newick());
//     x.balance_subtree();
//     dbg!(x.to_newick());
// }
