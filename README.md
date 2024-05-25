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

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/AbdelStark"><img src="https://avatars.githubusercontent.com/u/45264458?v=4?s=100" width="100px;" alt="Abdel @ StarkWare "/><br /><sub><b>Abdel @ StarkWare </b></sub></a><br /><a href="https://github.com/AbdelStark/majin-blob/commits?author=AbdelStark" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/apoorvsadana"><img src="https://avatars.githubusercontent.com/u/95699312?v=4?s=100" width="100px;" alt="apoorvsadana"/><br /><sub><b>apoorvsadana</b></sub></a><br /><a href="https://github.com/AbdelStark/majin-blob/commits?author=apoorvsadana" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/MSghais"><img src="https://avatars.githubusercontent.com/u/59928086?v=4?s=100" width="100px;" alt="MSG"/><br /><sub><b>MSG</b></sub></a><br /><a href="https://github.com/AbdelStark/majin-blob/commits?author=MSghais" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Mohiiit"><img src="https://avatars.githubusercontent.com/u/48082542?v=4?s=100" width="100px;" alt="Mohit Dhattarwal"/><br /><sub><b>Mohit Dhattarwal</b></sub></a><br /><a href="https://github.com/AbdelStark/majin-blob/commits?author=Mohiiit" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/prashansatanwar"><img src="https://avatars.githubusercontent.com/u/53948644?v=4?s=100" width="100px;" alt="Prashansa Tanwar"/><br /><sub><b>Prashansa Tanwar</b></sub></a><br /><a href="https://github.com/AbdelStark/majin-blob/commits?author=prashansatanwar" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!