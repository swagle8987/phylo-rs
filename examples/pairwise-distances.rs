#[cfg(feature = "parallel")]
use rayon::prelude::*;

use itertools::Itertools;
use std::fs::read_to_string;
use phylo::prelude::*;
use indicatif::{ProgressIterator, ProgressBar, ProgressStyle};
use phylo::tree::DemoTree;

#[cfg(feature = "parallel")]
fn main() {
    let trees = (1..11).progress().map(|x| read_to_string(format!("examples/pairwise-distances/r{x}-preprocessed.trees"))
            .unwrap()
            .lines()
            .enumerate()
            .map(|(y,z)| (x,y,DemoTree::from_newick(z.as_bytes()).unwrap()))
            .collect_vec()
        )
        .flatten()
        .collect_vec();

    let bar = ProgressBar::new((trees.len()*(trees.len()-1)/2) as u64);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} [eta: {eta}]")
    .unwrap()
    .progress_chars("##-"));

    trees.iter().combinations(2).par_bridge().map(|v| (v[0], v[1])).for_each(|(x,y)| {
        let out = format!("{}-{}-{}-{}-{}\n", x.0, y.0, x.1, y.1, x.2.ca(&y.2));
        println!("{}", out);
        bar.inc(1);
    });
    bar.finish();
}

#[cfg(not(feature = "parallel"))]
fn main() {
    let trees = (1..11).progress().map(|x| read_to_string(format!("examples/pairwise-distances/r{x}-preprocessed.trees"))
            .unwrap()
            .lines()
            .enumerate()
            .map(|(y,z)| (x,y,DemoTree::from_newick(z.as_bytes()).unwrap()))
            .collect_vec()
        )
        .flatten()
        .collect_vec();


    let bar = ProgressBar::new((trees.len()*(trees.len()-1)/2) as u64);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} [eta: {eta}]")
    .unwrap()
    .progress_chars("##-"));

    trees.iter().combinations(2).map(|v| (v[0], v[1])).for_each(|(x,y)| {
        let out = format!("{}-{}-{}-{}-{}\n", x.0, y.0, x.1, y.1, x.2.ca(&y.2));
        println!("{}", out);
        bar.inc(1);
    });
    bar.finish();
}
