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

        let (class_flag, nonce, number_of_storage_updates) = extract_bits(&info_word);
        
        let new_class_hash = if class_flag {
            i += 1;
            Some(data[i].clone())
        } else {
            None
        };

        i += 1;

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

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::fs;
    use num_bigint::BigUint;
    use std::{result, str::FromStr};
    use rstest::rstest;
    use crate::state_diffs::{ClassDeclaration, ContractUpdate, DataJson, StorageUpdate};

    #[rstest]
    #[case("18446744073709551617",false, 1, 1)] // hex: 10000000000000001
    #[case("18446744073709551616",false, 1, 0)] // hex: 10000000000000000
    #[case("6",false, 0, 6)] 
    #[case("340282366920938463481821351505477763072", true, 1, 0)] // hex: 100000000000000010000000000000000
    #[case("0", false, 0, 0)]
    #[case("340282366920938463463374607431768211455", false, u64::MAX, u64::MAX)] // hex: FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    #[case("680564733841876926926749214863536422911", true, u64::MAX, u64::MAX)] // hex: 1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    #[case("340282366920938486226656794389354915599", true, 1234, 9999)] // hex: 100000000000004D2000000000000270F
    fn test_extract_bits(
        #[case] info_word: BigUint,
        #[case] expected_class_flag: bool,
        #[case] expected_nonce: u64,
        #[case] expected_num_changes: u64
    ) {
        let (class_flag, nonce, num_changes) = extract_bits(&info_word);
        assert_eq!(class_flag, expected_class_flag);
        assert_eq!(nonce, expected_nonce);
        assert_eq!(num_changes, expected_num_changes);
    }
    
    // Function to convert values in a string Array to BigUint Array
    fn biguints_from_strings(values: &[&str]) -> Vec<BigUint> {
        values.iter().map(|&v| BigUint::from_str(v).unwrap()).collect()
    } 

    #[rstest]
    // Single Contract Update with One Class Declaration
    #[case( 
        &biguints_from_strings(&[
            "2", "1", "1", "1", "1", "1234", "1", "12", "34", "1", "56", "78"
        ]),  DataJson {
            state_update_size: 1, 
            state_update: vec![ContractUpdate {address: BigUint::from(1234u64), nonce: 0, number_of_storage_updates: 1, new_class_hash: None, storage_updates: vec![StorageUpdate{key: BigUint::from(12u64), value: BigUint::from(34u64)}]}],
            class_declaration_size: 1, 
            class_declaration: vec![ClassDeclaration {class_hash:BigUint::from(56u64),compiled_class_hash:BigUint::from(78u64)}]
        }
    )]
    // Single Contract Update with One Storage Update and no 
    #[case( 
        &biguints_from_strings(&[
            "2", "1", "1", "1", "1", "1234", "1", "12", "34", "0"
        ]),  DataJson {
            state_update_size: 1, 
            state_update: vec![ContractUpdate {address: BigUint::from(1234u64), nonce: 0, number_of_storage_updates: 1, new_class_hash: None, storage_updates: vec![StorageUpdate{key: BigUint::from(12u64), value: BigUint::from(34u64)}]}],
            class_declaration_size: 0, 
            class_declaration: vec![]
        }
    )]
    // Single Contract Update with New Class Hash and No Storage Updates
    #[case( 
        &biguints_from_strings(&[
            "2", "1", "1", "1", "1", "1234", "340282366920938463481821351505477763072", "5432", "0"
        ]),  DataJson {
            state_update_size: 1, 
            state_update: vec![ContractUpdate {address: BigUint::from(1234u64), nonce: 1, number_of_storage_updates: 0, new_class_hash: Some(BigUint::from(5432u64)), storage_updates: vec![]}],
            class_declaration_size: 0, 
            class_declaration: vec![]
        }
    )]
    // Single Contract Update with New Class Hash and One Storage Update
    #[case( 
        &biguints_from_strings(&[
            "2", "1", "1", "1", "1", "1234", "340282366920938568203987457954602287105", "5432", "12", "34", "0"
        ]),  DataJson {
            state_update_size: 1, 
            state_update: vec![ContractUpdate {address: BigUint::from(1234u64), nonce: 5678, number_of_storage_updates: 1, new_class_hash: Some(BigUint::from(5432u64)), storage_updates: vec![StorageUpdate{key: BigUint::from(12u64), value: BigUint::from(34u64)}]}],
            class_declaration_size: 0, 
            class_declaration: vec![]
        }
    )]
    // Single Contract Update with New Class Hash and Two Storage Updates
    #[case( 
        &biguints_from_strings(&[
            "2", "1", "1", "1", "1", "1234", "340282366920938568203987457954602287106", "5432", "12", "34", "56", "78", "0"
        ]),  DataJson {
            state_update_size: 1, 
            state_update: vec![ContractUpdate {address: BigUint::from(1234u64), nonce: 5678, number_of_storage_updates: 2, new_class_hash: Some(BigUint::from(5432u64)), storage_updates: vec![StorageUpdate{key: BigUint::from(12u64), value: BigUint::from(34u64)}, StorageUpdate{key: BigUint::from(56u64), value: BigUint::from(78u64)}]}],
            class_declaration_size: 0, 
            class_declaration: vec![]
        }
    )]
    // No Contract Updates or Class Declarations
    #[case( 
        &biguints_from_strings(&[
            "1", "1", "1", "1", "1", "0"
        ]),  DataJson {
            state_update_size: 0, 
            state_update: vec![],
            class_declaration_size: 0, 
            class_declaration: vec![]
        }
    )]
    // Multiple Class Declarations with No Contract Updates
    #[case( 
        &biguints_from_strings(&[
            "1", "1", "1", "1", "1", "2", "34","12", "23", "56"
        ]),  DataJson {
            state_update_size: 0, 
            state_update: vec![],
            class_declaration_size: 2, 
            class_declaration: vec![ClassDeclaration {class_hash:BigUint::from(34u64),compiled_class_hash:BigUint::from(12u64)}, ClassDeclaration {class_hash:BigUint::from(23u64),compiled_class_hash:BigUint::from(56u64)}]
        }
    )]

    fn test_parse_state_diffs(#[case] data: &[BigUint], #[case] expected_result: DataJson) {
        let result = parse_state_diffs(data);
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(
        &"0000000000000000000000000000000100000000000000010000000000000000".repeat(4096), 
        &biguints_from_strings(&[
            "340282366920938463481821351505477763072"; 4096
        ])

    )]
    #[case(
        &"00000000000000000000000000000001FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF".repeat(4096), 
        &biguints_from_strings(&[
            "680564733841876926926749214863536422911"; 4096
        ])

    )]
    #[case(
        &"0000000000000000000000000000000100000000000004D2000000000000270F".repeat(4096), 
        &biguints_from_strings(&[
            "340282366920938486226656794389354915599"; 4096
        ])

    )]
    #[case(
        &"0000000000000D99634EE36951FE43500B1805B3CF342D12FA160E18DAB27208".repeat(4096), 
        &biguints_from_strings(&[
            "21853026169818237947070859940682658839820493093261147223781896"; 4096
        ])

    )]
    #[case(
        &"341606E915B8FCA7D6908382709FF17BB8CEF91ADDDAF43467903D7A0F1759DC6D676B911577A1AA3E35042EFB29BA0F1A618ADB1273BE72A61A71604E895F83".repeat(4096/2), 
        &biguints_from_strings(&[
            "23559186456188288176746211509925988380451132083205062504931431952989141031388", "49484828141560354917085179895740570325324754012313231526408991898487008878467"
        ].repeat(4096/2))

    )]
    fn test_parse_str_to_blob_data(#[case] data: &str, #[case] expected_result: &Vec<BigUint>) {
        let result = parse_str_to_blob_data(data);
        assert_eq!(result, expected_result.clone());
    }

    #[rstest]
    #[case("src/testutils/blob_640641.txt", "src/testutils/blob_640641_output.txt")]
    #[case("src/testutils/blob_640644.txt", "src/testutils/blob_640644_output.txt")]
    #[case("src/testutils/blob_640646.txt", "src/testutils/blob_640646_output.txt")]
    #[case("src/testutils/blob_640647.txt", "src/testutils/blob_640647_output.txt")]
    #[case("src/testutils/blob_639404.txt", "src/testutils/blob_639404_output.txt")]
    fn test_parse_file_to_blob_data(#[case] file_path: &str, #[case] expected_output_file_path: &str) {
        let result = parse_file_to_blob_data(file_path);
        let expected_output: Vec<BigUint> = fs::read_to_string(expected_output_file_path).expect("Failed to read file").lines().into_iter()
                                                .map(|s| BigUint::from_str(&s).expect("Failed to parse BigUint"))
                                                .collect();
        assert_eq!(result,expected_output);
    }
} 