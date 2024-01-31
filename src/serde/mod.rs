use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json;

// Define the data structures
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractUpdate {
    address: BigUint,
    class_info_flag: u8,
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

    while i < data.len() {
        let address = data[i].clone();
        i += 1;
        let info_word = &data[i];
        i += 1;

        let class_info_flag = (info_word >> 63_u32).to_u8().unwrap();
        let nonce = ((info_word >> 64_u32) & BigUint::from(0xFFFFFFFFFFFFFFFF_u64))
            .to_u64()
            .unwrap(); // Convert to u64
        let number_of_storage_updates = (info_word & BigUint::from(0xFFFFFFFFFFFFFFFF_u64))
            .to_u64()
            .unwrap(); // Convert to u64

        let new_class_hash = if class_info_flag == 1 {
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
