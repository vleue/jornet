use base64::{engine::general_purpose::STANDARD, Engine};
use biscuit_auth::KeyPair;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new biscuit key
    GenerateBiscuitKey {
        /// Output as dhall format
        #[clap(short, long, value_parser)]
        dhall: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateBiscuitKey { dhall } => {
            let root = KeyPair::new();
            let private_key = root.private();
            let output = if dhall {
                format!("\"{}\"", STANDARD.encode(private_key.to_bytes()))
            } else {
                STANDARD.encode(private_key.to_bytes())
            };
            println!("{}", output);
        }
    }
}
