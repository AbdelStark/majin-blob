use num_bigint::BigUint;
use num_traits::Num;
use sbw::core::{blob_recover, BLOB_LEN};
use std::fs;

fn main() {
    // Read the blob data from a file
    let blob_data = parse_to_blob_data("./examples/blob/sn_blob_goerli.txt");

    // Recover the original data
    let original_data = blob_recover(blob_data);

    println!("Original data: {:?}", original_data);

    // Parse the original data into state diffs
    //let state_diffs = sbw::serde::parse_state_diffs(original_data.as_slice());

    // Serialize the state diffs into JSON
    //let state_diffs_json = sbw::serde::to_json(state_diffs.as_slice());

    //println!("{}", state_diffs_json);
}

/// Read a file and return a vector of `BigUint` representing the data.
/// # Arguments
/// * `file_path` - The path to the file.
/// # Returns
/// A vector of `BigUint` representing the data.
fn parse_to_blob_data(file_path: &str) -> Vec<BigUint> {
    let blob_hex = fs::read_to_string(file_path).expect("Failed to read file");
    let blob_hex = blob_hex.trim();
    (0..BLOB_LEN)
        .map(|i| BigUint::from_str_radix(&blob_hex[i * 64..(i + 1) * 64], 16).unwrap())
        .collect()
}
