use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_traits::Num;
use sbw::blob_recover;
use std::fs;
use std::str::FromStr;

lazy_static! {
    /// EIP-4844 BLS12-381 modulus.
    ///
    /// As defined in https://eips.ethereum.org/EIPS/eip-4844
    static ref EIP_4844_BLS_MODULUS: BigUint = BigUint::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();
    /// Generator of the group of evaluation points (EIP-4844 parameter).
    static ref GENERATOR: BigUint = BigUint::from_str(
        "39033254847818212395286706435128746857159659164139250548781411570340225835782",
    )
    .unwrap();
}

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
