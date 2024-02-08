<div align="center">
  <img src="docs/logo/logo - gradient bg.png" height="300"/>
</div>

## Overview

Utility features to play with EIP-4844 blobs for Starknet usage.

## Usage

### As a CLI

You can use the CLI to recover the original data from a blob file and parse it into state diffs.

#### Install the CLI

```sh
cargo install majin-blob
```

#### Use the CLI

```sh
# Recover the original data from a blob file
majin-blob recover ./examples/blob/sn_blob_goerli.txt
```

#### Help

```sh
majin-blob --help
```

### As a library

Add the following to your `Cargo.toml`:

```toml
[dependencies]
majin-blob-core = "0.1.0"
majin-blob-types = "0.1.0"
```

Then you can use the library as follows:

```rust
use majin_blob_core::blob;
use majin_blob_types::serde;

fn main() {
    // Read the blob data from a file
    let blob_data = serde::parse_file_to_blob_data("./examples/blob/sn_blob_goerli.txt");

    // Recover the original data
    let original_data = blob::recover(blob_data);

    // Parse the original data into state diffs
    let state_diffs = serde::parse_state_diffs(original_data.as_slice());

    // Serialize the state diffs into JSON
    let state_diffs_json = serde::to_json(state_diffs.as_slice());

    println!("{}", state_diffs_json);
}
```

### Use the REST API

You can use the REST API to recover the original data from a blob file and parse it into state diffs.

#### Start the REST API

```sh
cargo run --release -p majin-blob-rest-api
```

#### Recover the original data from a blob file

```sh
curl -X POST --data-binary "@./examples/blob/sn_blob_goerli.txt" http://127.0.0.1:3030/blob
```

## License

This project is licensed under the [MIT license](LICENSE).
