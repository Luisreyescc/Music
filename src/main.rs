mod model {
    pub mod miner;
}
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
   
    if args.len() < 2 {
        eprintln!("Usage: cargo run <directory>");
        std::process::exit(1);
    }

    let directory = &args[1];
    model::miner::extract(directory);
}
