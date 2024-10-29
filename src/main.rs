use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    name: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.name);
}
