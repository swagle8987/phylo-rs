extern crate clap;

use phylo::*;
use clap::{arg, Command};


fn main(){
    let matches = Command::new("Generalized suffix tree")
        .version("1.0")
        .author("Sriram Vijendran <vijendran.sriram@gmail.com>")
        .subcommand(Command::new("cophen-dist")
            .about("Build suffix tree index from reference fasta file")
            .arg(arg!(-n --norm <NORM> "nth norm")
                .value_parser(clap::value_parser!(usize))
            )
            .arg(arg!(--tree1 <TREE1> "Tree one in newick")
                .required(true)
                    .value_parser(clap::value_parser!(String))
            )
            .arg(arg!(--tree2 <TREE2> "Tree two in newick")
            .required(true)
                .value_parser(clap::value_parser!(String))
            )
        )
        .about("CLI tool for quick tree operations")
        .get_matches();

        match matches.subcommand(){
            Some(("cophen-dist",  sub_m)) => {
                let tree1 = sub_m.get_one::<String>("tree1").expect("required").as_str();
                let tree2 = sub_m.get_one::<String>("tree2").expect("required").as_str();
                println!("{}", tree1);
                println!("{}", tree2);
                // let tree: KGST<char, String> = build_tree(
                //     sub_m.get_one::<String>("source").expect("required").as_str(), 
                //     sub_m.get_one::<usize>("num").expect("required"), 
                //     sub_m.get_one::<usize>("depth").expect("required")
                // );
                // if sub_m.get_flag("network"){
                //     save_tree_edges(&tree, sub_m.get_one::<String>("source").expect("required").to_string());
                // }
                // if sub_m.get_flag("sim"){
                //     node_sim(&tree, sub_m.get_one::<String>("source").expect("required").to_string());
                // }
                // // else{
                // //     save_tree(&tree, sub_m.get_one::<String>("out").expect("required").to_string());
                // // }
            },
            _ => {
                println!("No option selected! Refer help page (-h flag)");
            }
        }
}