use std::process;

use clap::Parser;
use kvs::{Commands, Opt};

fn main() {
    let opt = Opt::parse();

    match opt.command {
        Some(Commands::Set { key, val }) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        Some(Commands::Get { key }) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        Some(Commands::Rm { key }) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        None => {
            println!("No command provided");
            process::exit(1);
        }
    }
}
