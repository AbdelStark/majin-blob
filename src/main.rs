use num_bigint::BigInt;
use num_traits::Num;
use sbw::{blob_recover, BLOB_LEN};
use std::fs;

fn main() {
    let blob_hex =
        fs::read_to_string("./examples/blob/sn_blob_goerli.txt").expect("Failed to read file");
    let blob_hex = blob_hex.trim();

    let blob_data: Vec<BigInt> = (0..BLOB_LEN)
        .map(|i| BigInt::from_str_radix(&blob_hex[i * 64..(i + 1) * 64], 16).unwrap())
        .collect();

    let original_data = blob_recover(blob_data);

    println!("{:?}", original_data);
}
