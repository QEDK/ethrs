//!The provider module provides all the APIs necessary to interact with EVM JSON-RPC nodes. The most important of which is the `Provider` struct.
//!See the [implementation](https://docs.rs/ethrs/*/ethrs/provider/struct.Provider.html) documentation for more details.
use lazy_static::lazy_static;
use primitive_types::U256;
use regex::Regex;
use reqwest;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fmt::Write;
use std::string::String;

///The `Provider` struct simply contains the RPC url, a `reqwest` client and default headers.
///## Example
///```rust
///use ethrs::provider::Provider;
///
///let provider = Provider::new("https://rpc.ankr.com/eth");
///```
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct Provider {
    url: String,
    client: reqwest::blocking::Client,
    headers: HeaderMap,
}

pub enum DefaultBlockParam {
    EARLIEST,
    FINALIZED,
    SAFE,
    LATEST,
    PENDING,
}

///The `RPCResponse` struct allows for deserialization of generic RPC requests that may either return an error or a single hash as a result.
#[derive(Deserialize, Debug)]
pub struct RPCResponse {
    error: Option<RPCError>,
    result: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RPCError {
    message: String,
}

///The `BlockRPCResponse` struct allows for deserialization of JSON-RPC requests that may either return an error or return a block as a result.
#[derive(Deserialize, Debug)]
pub struct BlockRPCResponse {
    error: Option<String>,
    result: Option<Block>,
}

///The `TxRPCResponse` struct allows for deserialization of JSON-RPC requests that may either return an error or return a transaction as a result.
#[derive(Deserialize, Debug)]
pub struct TxRPCResponse {
    error: Option<String>,
    result: Option<Transaction>,
}

///The `TxReceiptRPCResponse` struct allows for deserialization of JSON-RPC requests that may either return an error or return a transaction receipt as a result.
#[derive(Deserialize, Debug)]
pub struct TxReceiptRPCResponse {
    error: Option<String>,
    result: Option<TransactionReceipt>,
}

///The `BlockWithTxRPCResponse` struct allows for deserialization of JSON-RPC requests that may either return an error or return a block with transactions as a result.
#[derive(Deserialize, Debug)]
pub struct BlockWithTxRPCResponse {
    error: Option<String>,
    result: Option<BlockWithTx>,
}

///The `Block` struct allows for returning successfully deserialized blocks from JSON-RPC requests.
///## Example
///```rust
///use ethrs::provider::Provider;
///use std::error::Error;
///
///fn main() -> Result<(), Box<dyn Error>> {
///  let provider = Provider::new("https://rpc.sepolia.org");
///  assert!(provider
///    .get_block_by_number(
///    None, None,
///    )?
///    .is_some());
///    Ok(())
///}
///```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: Option<U256>,
    pub hash: Option<String>,
    pub parent_hash: String,
    pub nonce: Option<U256>,
    pub sha3_uncles: String,
    pub logs_bloom: Option<String>,
    pub transactions_root: String,
    pub state_root: String,
    pub receipts_root: String,
    pub miner: Option<String>,
    pub difficulty: U256,
    pub total_difficulty: Option<U256>,
    pub extra_data: String,
    pub size: U256,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: U256,
    pub transactions: Vec<String>,
    pub uncles: Vec<String>,
}

///The `Block` struct allows for returning successfully deserialized blocks with transactions from JSON-RPC requests.
///## Example
///```rust
///use ethrs::provider::Provider;
///use std::error::Error;
///
///fn main() -> Result<(), Box<dyn Error>> {
///  let provider = Provider::new("https://rpc.sepolia.org");
///  assert!(provider
///    .get_block_by_number_with_tx(
///    None, None,
///    )?
///    .is_some());
///    Ok(())
///}
///```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockWithTx {
    pub number: Option<U256>,
    pub hash: Option<String>,
    pub parent_hash: String,
    pub nonce: Option<U256>,
    pub sha3_uncles: String,
    pub logs_bloom: Option<String>,
    pub transactions_root: String,
    pub state_root: String,
    pub receipts_root: String,
    pub miner: Option<String>,
    pub difficulty: U256,
    pub total_difficulty: Option<U256>,
    pub extra_data: String,
    pub size: U256,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: U256,
    pub transactions: Vec<Transaction>,
    pub uncles: Vec<String>,
}

///The `Transaction` struct allows for returning successfully deserialized transactions from JSON-RPC requests.
///## Example
///```rust
///use ethrs::provider::Provider;
///use std::error::Error;
///
///fn main() -> Result<(), Box<dyn Error>> {
///  let provider = Provider::new("https://rpc.sepolia.org");
///  assert!(provider
///    .get_transaction_by_hash(
///    "0x6648b858a3d2b716d4c05c5d611844eb9827e2eea5bfc9db7a92187afd4d8c17"
///    )?
///    .is_some());
///    Ok(())
///}
///```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_hash: Option<String>,
    pub block_number: Option<U256>,
    pub from: String,
    pub gas: U256,
    pub gas_price: U256,
    pub hash: String,
    pub input: String,
    pub nonce: U256,
    pub to: Option<String>,
    pub transaction_index: Option<U256>,
    pub value: U256,
    pub v: String,
    pub r: String,
    pub s: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub transaction_hash: String,
    pub transaction_index: U256,
    pub block_hash: String,
    pub block_number: U256,
    pub from: String,
    pub to: Option<String>,
    pub cumulative_gas_used: U256,
    pub effective_gas_price: U256,
    pub gas_used: U256,
    pub contract_address: Option<String>,
    pub logs: Vec<Log>,
    pub logs_bloom: String,
    pub status: Option<U256>,
    pub root: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInput {
    pub from: String,
    pub to: Option<String>,
    pub gas: Option<U256>,
    pub gas_price: Option<U256>,
    pub value: Option<U256>,
    pub data: Option<String>,
    pub nonce: Option<U256>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallInput {
    pub from: Option<String>,
    pub to: String,
    pub gas: Option<U256>,
    pub gas_price: Option<U256>,
    pub value: Option<U256>,
    pub data: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub removed: bool,
    pub log_index: U256,
    pub transaction_index: U256,
    pub transaction_hash: String,
    pub block_hash: String,
    pub block_number: U256,
    pub address: String,
    pub data: String,
    pub topics: Vec<String>,
}

lazy_static! {
    static ref ADDRESS_REGEX: Regex = Regex::new(r"0x[0-9A-Fa-f]{40}").unwrap();
    static ref BLOCKHASH_REGEX: Regex = Regex::new(r"0x[0-9A-Fa-f]{64}").unwrap();
    static ref SLOT_REGEX: Regex = Regex::new(r"0x[0-9A-Fa-f]{1,64}").unwrap();
}

///The `Provider` module requires an HTTP(S) JSON-RPC URL and is responsible for handling all your JSON-RPC requests.
///## Example
///```rust
///use ethrs::provider::Provider;
///
///let provider = Provider::new("https://rpc.sepolia.org");
///```
impl Provider {
    ///The `Provider::new()` associated function takes an HTTP(S) JSON-RPC URL and returns a `Provider`
    ///instance to make your requests.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///
    ///let provider: Provider = Provider::new("https://rpc.sepolia.org");
    ///```
    pub fn new(_url: &str) -> Provider {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        Provider {
            url: _url.to_owned(),
            client: reqwest::blocking::Client::new(),
            headers: headers.clone(),
        }
    }

    ///The `gas_price()` function attempts to return the current block number as `Ok(u128)`. Returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .block_number()?
    ///      >= 2900000);
    ///  Ok(())
    ///}
    ///```
    pub fn block_number(&self) -> Result<u128, Box<dyn Error>> {
        let json: RPCResponse = self
            .client
            .post(&self.url)
            .body("{\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1,\"jsonrpc\":\"2.0\"}")
            .headers(self.headers.clone())
            .send()?
            .json()?;

        match json.error {
            Some(err) => Err(err.message.into()),
            None => Ok(u128::from_str_radix(
                json.result.unwrap().strip_prefix("0x").unwrap(),
                16,
            )?),
        }
    }

    ///The `gas_price()` function attempts to return the current gas price as `Ok(u128)`. Returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .gas_price()?
    ///      >= 7);
    ///  Ok(())
    ///}
    ///```
    pub fn gas_price(&self) -> Result<u128, Box<dyn Error>> {
        let json: RPCResponse = self
            .client
            .post(&self.url)
            .body("{\"method\":\"eth_gasPrice\",\"params\":[],\"id\":1,\"jsonrpc\":\"2.0\"}")
            .headers(self.headers.clone())
            .send()?
            .json()?;

        match json.error {
            Some(err) => Err(err.message.into()),
            None => Ok(u128::from_str_radix(
                json.result.unwrap().strip_prefix("0x").unwrap(),
                16,
            )?),
        }
    }

    ///The `get_code()` function takes an address, block param or block number, and attempts to return a deserialized balance as `Ok(u128)`. Returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_balance("0x0000000000000000000000000000000000000000", None, None)? // fetches the latest balance of this address
    ///      > 0);
    ///  Ok(())
    ///}
    ///```
    pub fn get_balance(
        &self,
        address: &str,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<u128, Box<dyn Error>> {
        match ADDRESS_REGEX.is_match(address) {
            true => {
                let mut payload = String::new();
                payload.push_str("{\"method\":\"eth_getBalance\",\"params\":[\"");
                payload.push_str(address);
                payload.push_str("\",\"");
                match block_param {
                    Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
                    Some(DefaultBlockParam::FINALIZED) => payload.push_str("finalized"),
                    Some(DefaultBlockParam::SAFE) => payload.push_str("safe"),
                    Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
                    Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
                    None => match block_number {
                        Some(block) => payload.push_str(&format!("0x{block:x}")),
                        None => payload.push_str("latest"),
                    },
                }

                payload.push_str("\"],\"id\":1,\"jsonrpc\":\"2.0\"}");

                let json: RPCResponse = self
                    .client
                    .post(&self.url)
                    .body(payload.clone())
                    .headers(self.headers.clone())
                    .send()?
                    .json()?;

                match json.error {
                    Some(err) => Err(err.message.into()),
                    None => Ok(u128::from_str_radix(
                        json.result.unwrap().strip_prefix("0x").unwrap(),
                        16,
                    )?),
                }
            }
            false => Err("Invalid address".into()),
        }
    }

    ///The `get_storage_at()` function takes an address, slot, block param or block number, and attempts to return a deserialized code hexstring as `Ok(String)`. Returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_storage_at("0x6f14c02fc1f78322cfd7d707ab90f18bad3b54f5", "0x0", None, None)? // fetches the latest code at this address
    ///      != "0x0");
    ///  Ok(())
    ///}
    ///```
    pub fn get_storage_at(
        &self,
        address: &str,
        slot: &str,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<String, Box<dyn Error>> {
        match ADDRESS_REGEX.is_match(address) {
            true => match SLOT_REGEX.is_match(slot) {
                true => {
                    let mut payload = String::new();
                    payload.push_str("{\"method\":\"eth_getStorageAt\",\"params\":[\"");
                    payload.push_str(address);
                    payload.push_str("\",\"");
                    payload.push_str(slot);
                    payload.push_str("\",\"");
                    match block_param {
                        Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
                        Some(DefaultBlockParam::FINALIZED) => payload.push_str("finalized"),
                        Some(DefaultBlockParam::SAFE) => payload.push_str("safe"),
                        Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
                        Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
                        None => match block_number {
                            Some(block) => payload.push_str(&format!("0x{block:x}")),
                            None => payload.push_str("latest"),
                        },
                    }
                    payload.push_str("\"],\"id\":1,\"jsonrpc\":\"2.0\"}");

                    let json: RPCResponse = self
                        .client
                        .post(&self.url)
                        .body(payload.clone())
                        .headers(self.headers.clone())
                        .send()?
                        .json()?;

                    match json.error {
                        Some(err) => Err(err.message.into()),
                        None => Ok(json.result.unwrap()),
                    }
                }
                false => Err("Invalid slot".into()),
            },
            false => Err("Invalid address".into()),
        }
    }

    ///The `get_code()` function takes an address, block param or block number, and attempts to return a deserialized string as `Ok(String)`. Returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_code("0x6f14c02fc1f78322cfd7d707ab90f18bad3b54f5", None, None)? // fetches the latest code at this address
    ///      != "0x0");
    ///  Ok(())
    ///}
    ///```
    pub fn get_code(
        &self,
        address: &str,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<String, Box<dyn Error>> {
        match ADDRESS_REGEX.is_match(address) {
            true => {
                let mut payload = String::new();
                payload.push_str("{\"method\":\"eth_getCode\",\"params\":[\"");
                payload.push_str(address);
                payload.push_str("\",\"");
                match block_param {
                    Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
                    Some(DefaultBlockParam::FINALIZED) => payload.push_str("finalized"),
                    Some(DefaultBlockParam::SAFE) => payload.push_str("safe"),
                    Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
                    Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
                    None => match block_number {
                        Some(block) => payload.push_str(&format!("0x{block:x}")),
                        None => payload.push_str("latest"),
                    },
                }

                payload.push_str("\"],\"id\":1,\"jsonrpc\":\"2.0\"}");

                let json: RPCResponse = self
                    .client
                    .post(&self.url)
                    .body(payload.clone())
                    .headers(self.headers.clone())
                    .send()?
                    .json()?;

                match json.error {
                    Some(err) => Err(err.message.into()),
                    None => Ok(json.result.unwrap()),
                }
            }
            false => Err("Invalid address".into()),
        }
    }

    ///The `get_transaction_count()` function takes an address, block param or block number, and attempts to return a deserialized integer as `Ok(u128)`. Returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_transaction_count("0xec65818ff0f8b071e587a0bbdbecc94de739b6ec", None, None)? // fetches the latest transaction count for this address
    ///      > 0);
    ///  Ok(())
    ///}
    ///```
    pub fn get_transaction_count(
        &self,
        address: &str,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<u128, Box<dyn Error>> {
        match ADDRESS_REGEX.is_match(address) {
            true => {
                let mut payload = String::new();
                payload.push_str("{\"method\":\"eth_getTransactionCount\",\"params\":[\"");
                payload.push_str(address);
                payload.push_str("\",\"");
                match block_param {
                    Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
                    Some(DefaultBlockParam::FINALIZED) => payload.push_str("finalized"),
                    Some(DefaultBlockParam::SAFE) => payload.push_str("safe"),
                    Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
                    Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
                    None => match block_number {
                        Some(block) => payload.push_str(&format!("0x{block:x}")),
                        None => payload.push_str("latest"),
                    },
                }

                payload.push_str("\"],\"id\":1,\"jsonrpc\":\"2.0\"}");

                let json: RPCResponse = self
                    .client
                    .post(&self.url)
                    .body(payload.clone())
                    .headers(self.headers.clone())
                    .send()?
                    .json()?;

                match json.error {
                    Some(err) => Err(err.message.into()),
                    None => Ok(u128::from_str_radix(
                        json.result.unwrap().strip_prefix("0x").unwrap(),
                        16,
                    )?),
                }
            }
            false => Err("Invalid address".into()),
        }
    }

    ///The `get_block_transaction_count_by_hash()` function takes a blockhash and attempts to return a deserialized integer as `Ok(Some(u128))`. Returns a `None` when blockhash is not mined and returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_block_transaction_count_by_hash("0x6c4925c897c45d377d8fb3ef59df7e0cf97604fc85b909bb806818368fdc6b07")? // fetches the latest transaction count for this address
    ///      == Some(5));
    ///  Ok(())
    ///}
    ///```
    pub fn get_block_transaction_count_by_hash(
        &self,
        block_hash: &str,
    ) -> Result<Option<u128>, Box<dyn Error>> {
        match BLOCKHASH_REGEX.is_match(block_hash) {
            true => {
                let mut payload = String::new();
                payload
                    .push_str("{\"method\":\"eth_getBlockTransactionCountByHash\",\"params\":[\"");
                payload.push_str(block_hash);
                payload.push_str("\"],\"id\":1,\"jsonrpc\":\"2.0\"}");

                let json: RPCResponse = self
                    .client
                    .post(&self.url)
                    .body(payload.clone())
                    .headers(self.headers.clone())
                    .send()?
                    .json()?;

                match json.error {
                    Some(err) => Err(err.message.into()),
                    None => match json.result {
                        Some(result) => Ok(Some(u128::from_str_radix(
                            result.strip_prefix("0x").unwrap(),
                            16,
                        )?)),
                        None => Ok(None),
                    },
                }
            }
            false => Err("Invalid block hash".into()),
        }
    }

    ///The `get_block_by_hash()` function takes a block hash and attempts to return a deserialized block *without transactions* as `Ok(Some(Block))`. If no such block exists, returns `Ok(None)` and returns an `Err()` on JSON-RPC errors. Pending blocks will have some fields serialized as `None` types.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_block_by_hash("0x7caebcb62b8fdd21673bcd7d3737f3e6dc18915e08ef3c868cb42aa78eb95d06")? // fetches the block by hash
    ///      .is_some());
    ///  Ok(())
    ///}
    ///```
    pub fn get_block_by_hash(&self, block_hash: &str) -> Result<Option<Block>, Box<dyn Error>> {
        match BLOCKHASH_REGEX.is_match(block_hash) {
            true => {
                let mut payload = String::new();
                match write!(payload, "{{\"method\":\"eth_getBlockByHash\",\"params\":[\"{block_hash}\",false],\"id\":1,\"jsonrpc\":\"2.0\"}}") {
                    Ok(_) => (),
                    Err(err) => return Err(err.into()),
                };

                let json: BlockRPCResponse = self
                    .client
                    .post(&self.url)
                    .body(payload.clone())
                    .headers(self.headers.clone())
                    .send()?
                    .json()?;

                match json.error {
                    Some(err) => Err(err.into()),
                    None => Ok(json.result),
                }
            }
            false => Err("Invalid block hash".into()),
        }
    }

    ///The `get_block_by_hash_with_tx()` function takes a block hash and attempts to return a deserialized block *with transactions* as `Ok(Some(BlockWithTx))`. If no such block exists, returns `Ok(None)` and returns an `Err()` on JSON-RPC errors. Pending blocks will have some fields serialized as `None` types.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_block_by_hash_with_tx("0x7caebcb62b8fdd21673bcd7d3737f3e6dc18915e08ef3c868cb42aa78eb95d06")? // fetches the block by hash with txs
    ///      .is_some());
    ///  Ok(())
    ///}
    ///```
    pub fn get_block_by_hash_with_tx(
        &self,
        block_hash: &str,
    ) -> Result<Option<BlockWithTx>, Box<dyn Error>> {
        match BLOCKHASH_REGEX.is_match(block_hash) {
            true => {
                let mut payload = String::new();
                match write!(payload, "{{\"method\":\"eth_getBlockByHash\",\"params\":[\"{block_hash}\",true],\"id\":1,\"jsonrpc\":\"2.0\"}}") {
                    Ok(_) => (),
                    Err(err) => return Err(err.into()),
                };
                let json: BlockWithTxRPCResponse = self
                    .client
                    .post(&self.url)
                    .body(payload.clone())
                    .headers(self.headers.clone())
                    .send()?
                    .json()?;

                match json.error {
                    Some(err) => Err(err.into()),
                    None => Ok(json.result),
                }
            }
            false => Err("Invalid block hash".into()),
        }
    }

    ///The `get_block_by_number()` function takes a default block param or block number and attempts to return a deserialized block *without transactions* as `Ok(Some(Block))`. If no such block exists, returns `Ok(None)` and returns an `Err()` on JSON-RPC errors. Pending blocks will have some fields serialized as `None` types.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_block_by_number(None, None)? // fetches the latest block
    ///      .is_some());
    ///  Ok(())
    ///}
    ///```
    pub fn get_block_by_number(
        &self,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<Option<Block>, Box<dyn Error>> {
        let mut payload = String::new();
        payload.push_str("{\"method\":\"eth_getBlockByNumber\",\"params\":[\"");
        match block_param {
            Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
            Some(DefaultBlockParam::FINALIZED) => payload.push_str("finalized"),
            Some(DefaultBlockParam::SAFE) => payload.push_str("safe"),
            Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
            Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
            None => match block_number {
                Some(block) => payload.push_str(&format!("0x{block:x}")),
                None => payload.push_str("latest"),
            },
        }

        payload.push_str("\",false],\"id\":1,\"jsonrpc\":\"2.0\"}");

        let json: BlockRPCResponse = self
            .client
            .post(&self.url)
            .body(payload.clone())
            .headers(self.headers.clone())
            .send()?
            .json()?;

        match json.error {
            Some(err) => Err(err.into()),
            None => Ok(json.result),
        }
    }

    ///The `get_block_by_number_with_tx()` function takes a default block param or block number and attempts to return a deserialized block *with transactions* as `Ok(Some(BlockWithTx))`. If no such block exists, returns `Ok(None)` and returns an `Err()` on JSON-RPC errors. Pending blocks will have some fields serialized as `None` types.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_block_by_number_with_tx(None, None)? // fetches the latest block
    ///      .is_some());
    ///  Ok(())
    ///}
    ///```
    pub fn get_block_by_number_with_tx(
        &self,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<Option<BlockWithTx>, Box<dyn Error>> {
        let mut payload = String::new();
        payload.push_str("{\"method\":\"eth_getBlockByNumber\",\"params\":[\"");
        match block_param {
            Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
            Some(DefaultBlockParam::FINALIZED) => payload.push_str("finalized"),
            Some(DefaultBlockParam::SAFE) => payload.push_str("safe"),
            Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
            Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
            None => match block_number {
                Some(block) => payload.push_str(&format!("0x{block:x}")),
                None => payload.push_str("latest"),
            },
        }

        payload.push_str("\",true],\"id\":1,\"jsonrpc\":\"2.0\"}");

        let json: BlockWithTxRPCResponse = self
            .client
            .post(&self.url)
            .body(payload.clone())
            .headers(self.headers.clone())
            .send()?
            .json()?;

        match json.error {
            Some(err) => Err(err.into()),
            None => Ok(json.result),
        }
    }

    ///The `get_transaction_by_hash()` function takes a transaction hash attempts to return a deserialized transaction as `Ok(Some(Transaction))`. If no such transaction exists, returns `Ok(None)` and returns an `Err()` on JSON-RPC errors. Pending transactions will have some fields serialized as `None` types.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_transaction_by_hash("0xfb09cfce0695a6843ee3ad5ed4505ca4c8fc0b32f33c1ee12548ba78f0ee52be")?
    ///      .is_some());
    ///  Ok(())
    ///}
    ///```
    pub fn get_transaction_by_hash(
        &self,
        txhash: &str,
    ) -> Result<Option<Transaction>, Box<dyn Error>> {
        match BLOCKHASH_REGEX.is_match(txhash) {
            true => {
                let mut payload = String::new();
                match write!(payload, "{{\"method\":\"eth_getTransactionByHash\",\"params\":[\"{txhash}\"],\"id\":1,\"jsonrpc\":\"2.0\"}}") {
                    Ok(_) => (),
                    Err(err) => return Err(err.into())
                }

                let json: TxRPCResponse = self
                    .client
                    .post(&self.url)
                    .body(payload.clone())
                    .headers(self.headers.clone())
                    .send()?
                    .json()?;

                match json.error {
                    Some(err) => Err(err.into()),
                    None => Ok(json.result),
                }
            }
            false => Err("Invalid txhash".into()),
        }
    }

    ///The `get_transaction_by_block_hash_and_index()` function takes a block hash and transaction index and attempts to return a deserialized transaction as `Ok(Some(Transaction))`. If no such transaction exists on the index, returns `Ok(None)` and returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_transaction_by_block_hash_and_index("0xc49f9290e07575fbcf91a9349721edaff45a6600add9281e48a2948f01c1d8d4", U256::from(1))? // fetches the block by hash and returns the tx at index 1
    ///      .is_some());
    ///  Ok(())
    ///}
    ///```
    pub fn get_transaction_by_block_hash_and_index(
        &self,
        block_hash: &str,
        idx: U256,
    ) -> Result<Option<Transaction>, Box<dyn Error>> {
        match BLOCKHASH_REGEX.is_match(block_hash) {
            true => {
                let mut payload = String::new();
                match write!(payload, "{{\"method\":\"eth_getTransactionByBlockHashAndIndex\",\"params\":[\"{block_hash}\",\"0x{idx:x}\"],\"id\":1,\"jsonrpc\":\"2.0\"}}") {
                    Ok(_) => (),
                    Err(err) => return Err(err.into())
                }

                let json: TxRPCResponse = self
                    .client
                    .post(&self.url)
                    .body(payload.clone())
                    .headers(self.headers.clone())
                    .send()?
                    .json()?;

                match json.error {
                    Some(err) => Err(err.into()),
                    None => Ok(json.result),
                }
            }
            false => Err("Invalid blockhash".into()),
        }
    }

    ///The `get_transaction_by_block_number_and_index()` function takes a block number and transaction index and attempts to return a deserialized transaction as `Ok(Some(Transaction))`. If no such transaction exists on the index, returns `Ok(None)` and returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_transaction_by_block_number_and_index(U256::from(2893800), U256::from(1))? // fetches the block by number and returns the tx at index 1
    ///      .is_some());
    ///  Ok(())
    ///}
    ///```
    pub fn get_transaction_by_block_number_and_index(
        &self,
        block_number: U256,
        idx: U256,
    ) -> Result<Option<Transaction>, Box<dyn Error>> {
        let mut payload = String::new();
        match write!(payload, "{{\"method\":\"eth_getTransactionByBlockNumberAndIndex\",\"params\":[\"0x{block_number:x}\",\"0x{idx:x}\"],\"id\":1,\"jsonrpc\":\"2.0\"}}") {
            Ok(_) => (),
            Err(err) => return Err(err.into())
        }

        let json: TxRPCResponse = self
            .client
            .post(&self.url)
            .body(payload.clone())
            .headers(self.headers.clone())
            .send()?
            .json()?;

        match json.error {
            Some(err) => Err(err.into()),
            None => Ok(json.result),
        }
    }

    ///The `get_transaction_receipt()` function takes transaction hash and attempts to return a deserialized transaction receipt as `Ok(Some(TransactionReceipt))`. If no such transaction exists, returns `Ok(None)` and returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::Provider;
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///    assert!(provider
    ///      .get_transaction_receipt("0x71d6059608006e73a233978ee092e7a2066b2556bc4a31dfe9be1f23328ce36a")?.is_some());
    ///  Ok(())
    ///}
    ///```
    pub fn get_transaction_receipt(
        &self,
        txhash: &str,
    ) -> Result<Option<TransactionReceipt>, Box<dyn Error>> {
        match BLOCKHASH_REGEX.is_match(txhash) {
            true => {
                let mut payload = String::new();
                match write!(payload, "{{\"method\":\"eth_getTransactionReceipt\",\"params\":[\"{txhash}\"],\"id\":1,\"jsonrpc\":\"2.0\"}}") {
                    Ok(_) => (),
                    Err(err) => return Err(err.into())
                }

                let json: TxReceiptRPCResponse = self
                    .client
                    .post(&self.url)
                    .body(payload.clone())
                    .headers(self.headers.clone())
                    .send()?
                    .json()?;

                match json.error {
                    Some(err) => Err(err.into()),
                    None => Ok(json.result),
                }
            }
            false => Err("Invalid txhash".into()),
        }
    }

    ///The `send_transaction()` function takes a transaction input struct, sends it and attempts to return a deserialized transaction hash as `Ok(String)`. If no such transaction exists, returns `Ok(0x0...)` and returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::{Provider, TransactionInput};
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///  let tx = TransactionInput {
    ///      from: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_owned(),
    ///      to: Some("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_owned()),
    ///      gas: Some(U256::from(21000)),
    ///      gas_price: Some(U256::from(1)),
    ///      value: Some(U256::from(1)),
    ///      data: Some("0xFF".to_owned()),
    ///      nonce: Some(U256::from(0)),
    ///  };
    ///  // the RPC call itself will fail because the account is not unlocked
    ///  assert!(provider.send_transaction(tx).is_err());
    ///  Ok(())
    ///}
    ///```
    pub fn send_transaction(&self, tx: TransactionInput) -> Result<String, Box<dyn Error>> {
        let mut payload = String::new();

        let tx_json = serde_json::to_string(&tx)?;

        match write!(payload, "{{\"method\":\"eth_sendTransaction\",\"params\":[{tx_json}],\"id\":1,\"jsonrpc\":\"2.0\"}}") {
            Ok(_) => (),
            Err(err) => return Err(err.into())
        }

        let json: RPCResponse = self
            .client
            .post(&self.url)
            .body(payload.clone())
            .headers(self.headers.clone())
            .send()?
            .json()?;

        match json.error {
            Some(err) => Err(err.message.into()),
            None => match json.result {
                Some(hash) => Ok(hash),
                None => Err("No txhash returned".into()),
            },
        }
    }

    ///The `call()` function takes a call input struct, sends it and attempts to return deserialized return data as `Ok(String)`. If no data is returned or a transaction is sent to an EOA, returns `Ok(0x0...)` and returns an `Err()` on JSON-RPC errors.
    ///## Example
    ///```rust
    ///use ethrs::provider::{Provider, CallInput};
    ///use ethrs::types::U256;
    ///use std::error::Error;
    ///
    ///fn main() -> Result<(), Box<dyn Error>> {
    ///  let provider = Provider::new("https://rpc.sepolia.org");
    ///  let tx = CallInput {
    ///      from: None,
    ///      to: "0xfd6470334498a1f26db0c5915b026670499b2632".to_owned(),
    ///      gas: None,
    ///      gas_price: None,
    ///      value: None,
    ///      data: Some("0xd800df5c".to_owned()),
    ///  };
    ///  assert_eq!(provider.call(tx, None, None)?, "0x00000000000000000000000000000000000000000000000000000000000003e8".to_owned());
    ///  Ok(())
    ///}
    ///```
    pub fn call(
        &self,
        tx: CallInput,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<String, Box<dyn Error>> {
        let mut payload = String::new();

        let tx_json = serde_json::to_string(&tx)?;

        payload.push_str("{\"method\":\"eth_call\",\"params\":[");
        payload.push_str(&tx_json);
        payload.push_str(",\"");
        match block_param {
            Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
            Some(DefaultBlockParam::FINALIZED) => payload.push_str("finalized"),
            Some(DefaultBlockParam::SAFE) => payload.push_str("safe"),
            Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
            Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
            None => match block_number {
                Some(block) => payload.push_str(&format!("0x{block:x}")),
                None => payload.push_str("latest"),
            },
        }
        payload.push_str("\"],\"id\":1,\"jsonrpc\":\"2.0\"}");

        let json: RPCResponse = self
            .client
            .post(&self.url)
            .body(payload.clone())
            .headers(self.headers.clone())
            .send()?
            .json()?;

        match json.error {
            Some(err) => Err(err.message.into()),
            None => match json.result {
                Some(data) => Ok(data),
                None => Err("No data returned".into()),
            },
        }
    }
}
