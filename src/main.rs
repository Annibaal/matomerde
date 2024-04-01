mod common {
    pub mod cheat_cache;
    pub mod log_reader;
    pub mod result;
}
mod config;
use std::time::Duration;

use clap::{builder::Str, Parser};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Path to the log file
    #[arg(short, long)]
    file_path: String,

    // Path to the config file
    #[arg(short, long)]
    conf_path: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.file_path)
}
