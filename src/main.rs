use num_bigint::BigUint;
use num_traits::Num;
use sbw::blob_recover;
use std::fs;

fn main() {
    let blob_hex =
        fs::read_to_string("./examples/blob/sn_blob_goerli.txt").expect("Failed to read file");
    let blob_hex = blob_hex.trim();

    let blob_data: Vec<BigUint> = (0..4096)
        .map(|i| BigUint::from_str_radix(&blob_hex[i * 64..(i + 1) * 64], 16).unwrap())
        .collect();

    let original_data = blob_recover(blob_data);

    println!("Original data: {:?}", original_data);
}
