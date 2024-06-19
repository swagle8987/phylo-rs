use phylo::tree::SimpleRootedTree;
use phylo::iter::node_iter::EulerWalk;

const NUM_TAXA: usize = 1000;

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