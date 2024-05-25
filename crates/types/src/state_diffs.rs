use num_bigint::BigUint;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DataJson {
    pub state_update_size: u64,
    pub state_update: Vec<ContractUpdate>,
    pub class_declaration_size: u64,
    pub class_declaration: Vec<ClassDeclaration>,
}

// Define the data structures

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContractUpdate {
    #[serde(serialize_with = "serialize_biguint")]
    pub address: BigUint,
    pub nonce: u64,
    pub number_of_storage_updates: u64,
    #[serde(serialize_with = "serialize_option_biguint")]
    pub new_class_hash: Option<BigUint>, // Present only if class_info_flag is 1
    pub storage_updates: Vec<StorageUpdate>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct StorageUpdate {
    #[serde(serialize_with = "serialize_biguint")]
    pub key: BigUint,
    #[serde(serialize_with = "serialize_biguint")]
    pub value: BigUint,
}

#[derive(Serialize, Deserialize, Debug, Clone,PartialEq, Eq, Hash)]
pub struct ClassDeclaration {
    #[serde(serialize_with = "serialize_biguint")]
    pub class_hash: BigUint,
    #[serde(serialize_with = "serialize_biguint")]
    pub compiled_class_hash: BigUint,
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

impl ContractUpdate {
    // Helper function to create a key for sorting
    fn sort_key(&self) -> BigUint {
        self.address.clone()
    }

    fn has_same_storage_updates(&self, other: &ContractUpdate) -> bool {
        let mut self_storage = self.storage_updates.clone();
        let mut other_storage = other.storage_updates.clone();

        // Sort the storage updates by the unique key
        self_storage.sort_by_key(|update| update.sort_key_storage());
        other_storage.sort_by_key(|update| update.sort_key_storage());

        if self_storage.len() != other_storage.len() {
            return false;
        }

        self_storage
            .iter()
            .zip(other_storage.iter())
            .all(|(self_update, other_update)| self_update == other_update)
    }
}

impl StorageUpdate {
    fn sort_key_storage(&self) -> BigUint {
        self.key.clone()
    }
}
impl PartialEq for ContractUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
            && self.nonce == other.nonce
            && self.number_of_storage_updates == other.number_of_storage_updates
            && self.new_class_hash == other.new_class_hash
            && self.has_same_storage_updates(other)
    }
}

impl DataJson {
    pub fn has_same_contract_updates(&self, other: &DataJson) -> bool {
        let mut self_updates = self.state_update.clone();
        let mut other_updates = other.state_update.clone();

        // Sort the updates by the unique identifier (address)
        self_updates.sort_by_key(|update| update.sort_key());
        other_updates.sort_by_key(|update| update.sort_key());

        if self_updates.len() != other_updates.len() {
            return false;
        }

        for (update_self, update_other) in self_updates.iter().zip(other_updates.iter()) {
            if update_self != update_other {
                return false;
            }
        }

        true
    }
}

pub fn have_identical_class_declarations(a: &DataJson, b: &DataJson) -> bool {
    let set_a: HashSet<_> = a.class_declaration.iter().cloned().collect();
    let set_b: HashSet<_> = b.class_declaration.iter().cloned().collect();

    set_a == set_b
}
