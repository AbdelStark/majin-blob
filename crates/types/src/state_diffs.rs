use num_bigint::BigUint;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
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
