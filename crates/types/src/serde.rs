use std::fs;

use crate::state_diffs::{ClassDeclaration, ContractUpdate, DataJson, StorageUpdate};
use majin_blob_eip_4844::BLOB_LEN;
use num_bigint::BigUint;
use num_traits::{Num, One, ToPrimitive, Zero};
use serde_json;

/// Function to parse the encoded data into a vector of StateDiff structs.
/// # Arguments
/// * `data` - A vector of `BigUint` representing the encoded data.
/// # Returns
/// A vector of `ContractUpdate` structs.
pub fn parse_state_diffs(data: &[BigUint]) -> DataJson {
    let mut updates = Vec::new();
    let mut i = 0;
    let contract_updated_num = data[i].to_usize().unwrap();
    i += 5;

    for _ in 0..contract_updated_num - 1 {
        let address = data[i].clone();
        // Break if address undefined
        if address == BigUint::zero() {
            break;
        }
        i += 1;
        // Break after blob data len
        if i >= BLOB_LEN - 1 {
            break;
        }
        let info_word = &data[i];
        i += 1;

        let (class_flag, nonce, number_of_storage_updates) = extract_bits_v2(&info_word);

        // TODO verify info_word len
        // let class_info_flag = extract_bits(&info_word, 0, 1);
        let new_class_hash = if class_flag {
            i += 1;
            Some(data[i].clone())
        } else {
            None
        };

        // Nonce are the next 64 bits
        // TODO verify info_word len
        // let nonce = extract_bits(&info_word, 1, 65).to_u64().unwrap();
        // Number of storage updates are the next 64 bits
        // TODO verify info_word len
        // let number_of_storage_updates = extract_bits(&info_word, 66, 129).to_u64().unwrap();

        let mut storage_updates = Vec::new();
        for _ in 0..number_of_storage_updates {
            // Break after blob data len
            if i >= BLOB_LEN - 1 {
                break;
            }
            let key = data[i].clone();
            i += 1;
            let value = data[i].clone();
            i += 1;
            // TODO verify key/value if null or 0
            if key == BigUint::zero() && value == BigUint::zero() {
                break;
            }
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

    let declared_classes_len = data[i].to_usize().unwrap();
    let mut class_declaration_updates = Vec::new();
    i += 1;
    for _ in 0..declared_classes_len {
        let class_hash = data[i].clone();
        // Break if address undefined
        if class_hash == BigUint::zero() {
            println!("no class hash declared ser!!!");
            break;
        }
        i += 1;
        // Break after blob data len
        if i >= BLOB_LEN - 1 {
            println!("the loop ends here becayse of length ");
            break;
        }
        let compiled_class_hash = data[i].clone();
        i += 1;

        class_declaration_updates.push(ClassDeclaration {
            class_hash,
            compiled_class_hash,
        });
    }

    let final_result = DataJson {
        state_update_size: (contract_updated_num - 1).to_u64().unwrap(),
        state_update: updates,
        class_declaration_size: declared_classes_len.to_u64().unwrap(),
        class_declaration: class_declaration_updates,
    };

    final_result
}

/// Function to convert a vector of StateDiff structs into a JSON string.
/// # Arguments
/// * `state_diffs` - A vector of `StateDiff` structs.
/// # Returns
/// A JSON string.
pub fn to_json(state_diffs: DataJson) -> String {
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
/// @TODO: Implement a more efficient way to extract bits.
// Verify bits len and more
fn extract_bits(word: &BigUint, start: usize, end: usize) -> BigUint {
    let string = format!("{:#b}", word).replace("0b", "");
    // TODO add check before  call extract_bits?
    if string.len() < end {
        let bit_string: String = format!("{:#b}", word).replace("0b", "");
        // 0 index and end max
        let bit_string = bit_string[0..string.len()].parse::<String>().unwrap();
        let bits = BigUint::from_str_radix(&bit_string, 2).unwrap_or_default();
        bits
    } else {
        let bit_string: String = format!("{:#b}", word).replace("0b", "");
        let bit_string = bit_string[start..end].parse::<String>().unwrap_or_default();
        let bits = BigUint::from_str_radix(&bit_string, 2).unwrap_or_default();
        bits
    }
}

fn extract_bits_v2(info_word: &BigUint) -> (bool, u64, u64) {
    let binary_string = format!("{:b}", info_word);
    let bitstring = format!("{:0>256}", binary_string);
    if bitstring.len() != 256 {
        panic!("Input string must be 256 bits long");
    }

    let class_flag_bit = &bitstring[127..128];
    let new_nonce_bits = &bitstring[128..192];
    let num_changes_bits = &bitstring[192..256];

    let class_flag = class_flag_bit == "1";
    let new_nonce =
        u64::from_str_radix(new_nonce_bits, 2).expect("Invalid binary string for new nonce");
    let num_changes =
        u64::from_str_radix(num_changes_bits, 2).expect("Invalid binary string for num changes");

    (class_flag, new_nonce, num_changes)
}
