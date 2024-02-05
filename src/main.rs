fn main() {
    // Read the blob data from a file
    let blob_data =
        majin_blob::serde::parse_file_to_blob_data("./examples/blob/sn_blob_goerli.txt");

    // Recover the original data
    let original_data = majin_blob::blob::recover(blob_data);

    println!("Original data: {:?}", original_data);

    // Parse the original data into state diffs
    let state_diffs = majin_blob::serde::parse_state_diffs(original_data.as_slice());

    // Serialize the state diffs into JSON
    let state_diffs_json = majin_blob::serde::to_json(state_diffs.as_slice());

    println!("{}", state_diffs_json);
}
