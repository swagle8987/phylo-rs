use phylo::tree::distances::PathFunction;
use phylo::tree::{SimpleRootedTree, simple_rtree::RootedTree};
use phylo::iter::node_iter::EulerWalk;
use phylo::tree::ops::CopheneticDistance;

const NUM_TAXA: usize = 1000;
const NORM: u32 = 1;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn benchmark_lca(bencher: divan::Bencher) {
    let mut tree = SimpleRootedTree::yule(NUM_TAXA).unwrap();
    tree.precompute_constant_time_lca();
    bencher.bench(||   tree.get_lca_id(&vec![10, 20]));
}

#[divan::bench]
fn benchmark_yule(bencher: divan::Bencher) {
    bencher.bench(|| SimpleRootedTree::yule(NUM_TAXA));
}

#[divan::bench]
fn benchmark_precompute_rmq(bencher: divan::Bencher) {
    bencher.with_inputs(|| {
        SimpleRootedTree::yule(NUM_TAXA).unwrap()
    })
    .bench_refs(|tree| {
        tree.precompute_constant_time_lca();
    });
}

#[divan::bench]
fn benchmark_cophen_dist_naive(bencher: divan::Bencher) {
    bencher.with_inputs(|| {
        fn depth(tree: &SimpleRootedTree, node_id: <SimpleRootedTree as RootedTree>::NodeID)->f32
        {
            EulerWalk::get_node_depth(tree, node_id) as f32
        }

        let mut t1 = SimpleRootedTree::yule(NUM_TAXA).unwrap();
        let mut t2 = SimpleRootedTree::yule(NUM_TAXA).unwrap();
        t1.precompute_constant_time_lca();
        t2.precompute_constant_time_lca();
        t1.set_zeta(depth);
        t2.set_zeta(depth);
        (t1, t2)
    })
    .bench_refs(|(t1, t2)| {
        t1.cophen_dist_naive(t2, NORM);
    });
}