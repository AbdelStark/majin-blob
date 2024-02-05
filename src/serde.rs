use std::fs;

use crate::eip4844_params::BLOB_LEN;
use num_bigint::BigUint;
use num_traits::{Num, One, ToPrimitive};
use serde::{Deserialize, Serialize, Serializer};
use serde_json;

// Define the data structures
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractUpdate {
    #[serde(serialize_with = "serialize_biguint")]
    address: BigUint,
    nonce: u64,
    number_of_storage_updates: u64,
    #[serde(serialize_with = "serialize_option_biguint")]
    new_class_hash: Option<BigUint>, // Present only if class_info_flag is 1
    storage_updates: Vec<StorageUpdate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageUpdate {
    #[serde(serialize_with = "serialize_biguint")]
    key: BigUint,
    #[serde(serialize_with = "serialize_biguint")]
    value: BigUint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassDeclaration {
    #[serde(serialize_with = "serialize_biguint")]
    class_hash: BigUint,
    #[serde(serialize_with = "serialize_biguint")]
    compiled_class_hash: BigUint,
}

// Custom serializer for BigUint
fn serialize_biguint<S>(biguint: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&biguint.to_str_radix(10))
}

// Custom serializer for Option<BigUint>
fn serialize_option_biguint<S>(
    option_biguint: &Option<BigUint>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match option_biguint {
        Some(value) => serialize_biguint(value, serializer),
        None => serializer.serialize_none(),
    }
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
        i += 1;
        let info_word = &data[i];
        i += 1;

        let class_info_flag = extract_bits(&info_word, 0, 1);
        let new_class_hash = if class_info_flag == BigUint::one() {
            i += 1;
            Some(data[i].clone())
        } else {
            None
        };

        // Nonce are the next 64 bits
        let nonce = extract_bits(&info_word, 1, 65).to_u64().unwrap();
        // Number of storage updates are the next 64 bits
        let number_of_storage_updates = extract_bits(&info_word, 66, 129).to_u64().unwrap();

        let mut storage_updates = Vec::new();
        for _ in 0..number_of_storage_updates {
            let key = data[i].clone();
            i += 1;
            let value = data[i].clone();
            i += 1;
            storage_updates.push(StorageUpdate { key, value });
        }

        updates.push(ContractUpdate {
            address,
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

/// Function to extract bits from a `BigUint` and return a new `BigUint`.
/// # Arguments
/// * `word` - The `BigUint` to extract bits from.
/// * `start` - The start index of the bits to extract.
/// * `end` - The end index of the bits to extract.
/// # Returns
/// A new `BigUint` representing the extracted bits.
fn extract_bits(word: &BigUint, start: usize, end: usize) -> BigUint {
    let bit_string = format!("{:#b}", word).replace("0b", "");
    let bit_string = bit_string[start..end].parse::<String>().unwrap();
    BigUint::from_str_radix(&bit_string, 2).unwrap()
}
