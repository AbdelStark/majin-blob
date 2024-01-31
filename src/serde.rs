use std::fs;

use crate::eip4844_params::BLOB_LEN;
use bitvec::{order::Lsb0, vec::BitVec};
use num_bigint::BigUint;
use num_traits::{Num, One, ToPrimitive};
use serde::{Deserialize, Serialize};
use serde_json;

// Define the data structures
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractUpdate {
    address: BigUint,
    class_info_flag: BigUint,
    nonce: u64,
    number_of_storage_updates: u64,
    new_class_hash: Option<BigUint>, // Present only if class_info_flag is 1
    storage_updates: Vec<StorageUpdate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageUpdate {
    key: BigUint,
    value: BigUint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassDeclaration {
    class_hash: BigUint,
    compiled_class_hash: BigUint,
}

/// Function to parse the encoded data into a vector of StateDiff structs.
/// # Arguments
/// * `data` - A vector of `BigUint` representing the encoded data.
/// # Returns
/// A vector of `ContractUpdate` structs.
pub fn parse_state_diffs(data: &[BigUint]) -> Vec<ContractUpdate> {
    let mut updates = Vec::new();
    let mut i = 0;
    let contract_updated_num = data[i].to_usize().unwrap();
    i += 1;

    for _ in 0..contract_updated_num {
        let address = data[i].clone();
        println!("address: {}", address);
        i += 1;
        let info_word = &data[i];
        println!("info_word: {}", info_word);
        i += 1;
        //let class_info_flag = info_word >> 63_u32;

        let class_info_flag_bits: BitVec<_, Lsb0> = BitVec::from_vec(info_word.to_bytes_be());
        println!("class_info_flag_bits: {:?}", class_info_flag_bits.len());
        println!("{}", class_info_flag_bits);

        // Extract class info flag (last bit)
        //let class_info_flag = (info_word & BigUint::one()).to_u8().unwrap();
        // Extract nonce (next 64 bits)
        // let nonce = (info_word >> BigUint::one() & BigUint::from(0xFFFFFFFFFFFFFFFF_u64))
        //     .to_u64()
        //     .unwrap();
        // Extract number of storage updates (next 64 bits)
        // let number_of_storage_updates = (info_word >> 65 & BigUint::from(0xFFFFFFFFFFFFFFFF_u64))
        //     .to_u64()
        //     .unwrap();
        let class_info_flag = BigUint::one();
        let nonce = 1_u64;
        let number_of_storage_updates = 1_u64;
        println!("class_info_flag: {}", class_info_flag);
        println!("nonce: {}", nonce);
        println!("number_of_storage_updates: {}", number_of_storage_updates);

        let new_class_hash = if class_info_flag == BigUint::one() {
            i += 1;
            Some(data[i].clone())
        } else {
            None
        };

        let mut storage_updates = Vec::new();
        for _ in 0..number_of_storage_updates {
            i += 1;
            let key = data[i].clone();
            i += 1;
            let value = data[i].clone();
            storage_updates.push(StorageUpdate { key, value });
        }

        updates.push(ContractUpdate {
            address,
            class_info_flag,
            nonce,
            number_of_storage_updates,
            new_class_hash,
            storage_updates,
        });
    }

    updates
}

/// Function to convert a vector of StateDiff structs into a JSON string.
/// # Arguments
/// * `state_diffs` - A vector of `StateDiff` structs.
/// # Returns
/// A JSON string.
pub fn to_json(state_diffs: &[ContractUpdate]) -> String {
    serde_json::to_string_pretty(&state_diffs).unwrap()
}

/// Read a file and return a vector of `BigUint` representing the data.
/// # Arguments
/// * `file_path` - The path to the file.
/// # Returns
/// A vector of `BigUint` representing the data.
pub fn parse_file_to_blob_data(file_path: &str) -> Vec<BigUint> {
    let blob_hex = fs::read_to_string(file_path).expect("Failed to read file");
    parse_str_to_blob_data(blob_hex.as_str())
}

/// Parse a string and return a vector of `BigUint` representing the data.
/// # Arguments
/// * `data` - The string to parse.
/// # Returns
/// A vector of `BigUint` representing the data.
pub fn parse_str_to_blob_data(data: &str) -> Vec<BigUint> {
    let blob_hex = data.trim();
    (0..BLOB_LEN)
        .map(|i| BigUint::from_str_radix(&blob_hex[i * 64..(i + 1) * 64], 16).unwrap())
        .collect()
}
