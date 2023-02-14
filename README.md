# ethrs [![Rust CI](https://github.com/QEDK/ethrs/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/QEDK/ethrs/actions/workflows/rust.yml)
An opinionated and blazing-fast crate for interacting with the EVM ⚡️
This crate tries to simplify the work involved with serializing and deserializing, mostly choosing to default to `String`, `U256`, and `uint128` types. The choice is intentional and prevents assumptions regarding deserialized data.

⚠️ ***This crate is still in `beta` and will not follow semver until a production release. It is recommended that you pin the crate when using it to ensure that non-backward compatible changes do not affect you.***

### 🧰 Installation
You can install this crate easily via `cargo` by running the command:
```bash
cargo add ethrs
```
or, add it manually in your `Cargo.toml` file like:
```TOML
[dependencies]
ethrs = "0.1.1"
```

## 🚀 Quick start
```rust
use ethrs::provider::Provider;
use ethrs::provider::Block;
use ethrs::provider::DefaultBlockParam;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let provider = Provider::new("https://rpc.ankr.com/eth");
    // Get the latest block number
    print!("Latest block number: {}", provider.block_number().unwrap());
    // Or fetch a pending block
    let pending_block: Block = provider.get_block_by_number(Some(DefaultBlockParam::PENDING), None)?.unwrap();
    // More APIs available in the docs!
    Ok(())
}
```

## 📜 License

Licensed under either of:

 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Apache-2.0 License ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option.

### ✏ Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
