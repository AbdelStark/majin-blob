mod utils;

use majin_blob_core::blob::recover;
use majin_blob_types::serde::{parse_state_diffs, parse_str_to_blob_data, to_json};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn blob_recover(data: &str) -> String {
    let blob_data = parse_str_to_blob_data(data);
    let original_data = recover(blob_data);
    let state_diffs = parse_state_diffs(&original_data);
    let state_diffs_json = to_json(state_diffs);
    state_diffs_json
}
