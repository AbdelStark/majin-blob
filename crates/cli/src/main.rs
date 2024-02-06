use majin_blob_core::blob;
use majin_blob_types::serde;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Recover the original data from the blob data
    Recover {
        /// The file containing the blob data
        #[arg(short, long, value_name = "FILE", required = true)]
        blob_file: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Recover { blob_file }) => {
            let blob_file = blob_file.unwrap();
            let blob_data = serde::parse_file_to_blob_data(blob_file.to_str().unwrap());
            let original_data = blob::recover(blob_data);
            let state_diffs = serde::parse_state_diffs(original_data.as_slice());
            let state_diffs_json = serde::to_json(state_diffs.as_slice());
            println!("{}", state_diffs_json);
        }
        None => {
            println!("No command provided");
        }
    }
}
