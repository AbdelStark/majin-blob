use std::fs;

use crate::state_diffs::{ClassDeclaration, ContractUpdate, DataJson, StorageUpdate};
use majin_blob_eip_4844::BLOB_LEN;
use num_bigint::BigUint;
use num_traits::{Num, ToPrimitive, Zero};
use serde_json;

/// Function to parse the encoded data into a vector of StateDiff structs.
/// # Arguments
/// * `data` - A vector of `BigUint` representing the encoded data.
/// # Returns
/// A `DataJson` structs.
pub fn parse_state_diffs(data: &[BigUint]) -> DataJson {
    let mut updates = Vec::new();
    let mut i = 0;
    let contract_updated_num = data[i].to_usize().unwrap();
    i += 5;
    // iterate only on len-1 because (len-1)th element contains the length
    // of declared classes.
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

        let (class_flag, nonce, number_of_storage_updates) = extract_bits(&info_word);

        let new_class_hash = if class_flag {
            i += 1;
            Some(data[i].clone())
        } else {
            None
        };

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
            panic!("class hash can't be zero when the len of declared_classes is non-zero");
            break;
        }
        i += 1;
        // Break after blob data len
        if i >= BLOB_LEN - 1 {
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

/// Function to extract class flag, nonce and state_diff length from a `BigUint`.
/// # Arguments
/// * `info_word` - The `BigUint` to extract bits from.
/// # Returns
/// A `bool` representing the class flag.
/// A `u64` representing the nonce.
/// Another`u64` representing the state_diff length
fn extract_bits(info_word: &BigUint) -> (bool, u64, u64) {
    // converting the bigUint to binary
    let binary_string = format!("{:b}", info_word);
    // adding padding so that it can be of 256 length
    let bitstring = format!("{:0>256}", binary_string);
    if bitstring.len() != 256 {
        panic!("Input string must be 256 bits long");
    }
    // getting the class flag, 127th bit is class flag (assuming 0 indexing)
    let class_flag_bit = &bitstring[127..128];
    // getting the nonce, nonce is of 64 bit from 128th bit to 191st bit
    let new_nonce_bits = &bitstring[128..192];
    // getting the state_diff_len, state_diff_len is of 64 bit from 192nd bit to 255th bit
    let num_changes_bits = &bitstring[192..256];

    // converting data to respective type
    let class_flag = class_flag_bit == "1";
    let new_nonce =
        u64::from_str_radix(new_nonce_bits, 2).expect("Invalid binary string for new nonce");
    let num_changes =
        u64::from_str_radix(num_changes_bits, 2).expect("Invalid binary string for num changes");

    (class_flag, new_nonce, num_changes)
}
