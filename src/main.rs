use clap::Parser;

use crate::args::Args;

pub mod args;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
