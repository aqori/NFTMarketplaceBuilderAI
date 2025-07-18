// src/main.rs
/*
 * Main executable for NFTMarketplaceBuilderAI
 */

use clap::Parser;
use nftmarketplacebuilderai::{Result, run};

#[derive(Parser)]
#[command(version, about = "NFTMarketplaceBuilderAI - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
