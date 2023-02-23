///## 🚀 Quick start
///```rust
///use ethrs::provider::Provider;
///use ethrs::provider::Block;
///use ethrs::provider::DefaultBlockParam;
///use std::error::Error;

///fn main() -> Result<(), Box<dyn Error>> {
///    let provider = Provider::new("https://rpc.sepolia.org");
///    // Get the latest block number
///    print!("Latest block number: {}", provider.block_number().unwrap());
///    // Or fetch a pending block
///    let pending_block: Block = provider.get_block_by_number(Some(DefaultBlockParam::PENDING), None)?.unwrap();
///    // More APIs available in the docs!
///    Ok(())
///}
///```
pub mod provider;
pub mod types;
