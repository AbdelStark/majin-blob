use num_bigint::BigUint;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DataJson {
    pub state_update_size: u64,
    pub state_update: Vec<ContractUpdate>,
    pub class_declaration_size: u64,
    pub class_declaration: Vec<ClassDeclaration>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

// Trait for unordered equality
pub trait UnorderedEq {
    fn unordered_eq(&self, other: &Self) -> bool;
}

// Implement UnorderedEq for DataJson
impl UnorderedEq for DataJson {
    fn unordered_eq(&self, other: &Self) -> bool {
        self.state_update.unordered_eq(&other.state_update)
            && self
                .class_declaration
                .unordered_eq(&other.class_declaration)
    }
}

// Implement UnorderedEq for Vec<ContractUpdate>
impl UnorderedEq for Vec<ContractUpdate> {
    fn unordered_eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut self_sorted = self.clone();
        let mut other_sorted = other.clone();

        self_sorted.sort_by_key(|update| update.address.clone());
        other_sorted.sort_by_key(|update| update.address.clone());

        for (self_update, other_update) in self_sorted.iter().zip(other_sorted.iter()) {
            if !self_update.unordered_eq(other_update) {
                return false;
            }
        }

        true
    }
}

// Implement UnorderedEq for Vec<ClassDeclaration>
impl UnorderedEq for Vec<ClassDeclaration> {
    fn unordered_eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let set_self: HashSet<_> = self.iter().collect();
        let set_other: HashSet<_> = other.iter().collect();

        set_self == set_other
    }
}

// Implement UnorderedEq for Vec<StorageUpdate>
impl UnorderedEq for Vec<StorageUpdate> {
    fn unordered_eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut self_sorted = self.clone();
        let mut other_sorted = other.clone();

        self_sorted.sort_by_key(|update| update.key.clone());
        other_sorted.sort_by_key(|update| update.key.clone());

        self_sorted == other_sorted
    }
}

// Implement UnorderedEq for ContractUpdate
impl UnorderedEq for ContractUpdate {
    fn unordered_eq(&self, other: &Self) -> bool {
        self.address == other.address
            && self.nonce == other.nonce
            && self.number_of_storage_updates == other.number_of_storage_updates
            && self.new_class_hash == other.new_class_hash
            && self.storage_updates.unordered_eq(&other.storage_updates)
    }
}

// Implement UnorderedEq for ClassDeclaration
impl UnorderedEq for ClassDeclaration {
    fn unordered_eq(&self, other: &Self) -> bool {
        self.class_hash == other.class_hash && self.compiled_class_hash == other.compiled_class_hash
    }
}
