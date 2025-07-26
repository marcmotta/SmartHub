// src/main.rs
/*
 * Main executable for SmartHub
 */

use clap::Parser;
use smarthub::{Result, run};

#[derive(Parser)]
#[command(version, about = "SmartHub - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
