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

    if let Some(Commands::Recover { blob_file }) = cli.command {
        let blob_file = blob_file.unwrap();
        let blob_data = serde::parse_file_to_blob_data(blob_file.to_str().unwrap());
        let original_data = blob::recover(blob_data);
        let state_diffs = serde::parse_state_diffs(original_data.as_slice());
        let state_diffs_json = serde::to_json(state_diffs);
        println!("state_diffs_json {}", state_diffs_json);
    }
}

#[test]
fn test_cli_sn_goerli() {
    let blob_data = serde::parse_file_to_blob_data("../../examples/blob/sn_blob_goerli.txt");
    let original_data = blob::recover(blob_data);

    let state_diffs = serde::parse_state_diffs(original_data.as_slice());
    let state_diffs_json = serde::to_json(state_diffs);
    println!("{}", state_diffs_json);
    // TODO assert result of old version of sn_goerli
}
