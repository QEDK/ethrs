use lazy_static::lazy_static;
use regex::Regex;
use reqwest;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::string::String;

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct Provider {
    url: String,
    client: reqwest::blocking::Client,
    headers: HeaderMap,
}

pub enum DefaultBlockParam {
    EARLIEST,
    LATEST,
    PENDING,
}

#[derive(Deserialize, Debug)]
pub struct RPCResponse {
    error: Option<String>,
    result: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BlockRPCResponse {
    error: Option<String>,
    result: Option<Block>,
}

#[derive(Deserialize, Debug)]
pub struct BlockWithTxRPCResponse {
    error: Option<String>,
    result: Option<BlockWithTx>,
}

#[derive(Deserialize, Debug)]
pub struct Block {
    number: Option<String>,
    hash: Option<String>,
    parentHash: String,
    nonce: Option<String>,
    sha3Uncles: String,
    logsBloom: Option<String>,
    transactionsRoot: String,
    stateRoot: String,
    receiptsRoot: String,
    miner: Option<String>,
    difficulty: String,
    totalDifficulty: Option<String>,
    extraData: String,
    size: String,
    gasLimit: String,
    gasUsed: String,
    timestamp: String,
    transactions: Vec<String>,
    uncles: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct BlockWithTx {
    number: Option<String>,
    hash: Option<String>,
    parentHash: String,
    nonce: Option<String>,
    sha3Uncles: String,
    logsBloom: Option<String>,
    transactionsRoot: String,
    stateRoot: String,
    receiptsRoot: String,
    miner: Option<String>,
    difficulty: String,
    totalDifficulty: Option<String>,
    extraData: String,
    size: String,
    gasLimit: String,
    gasUsed: String,
    timestamp: String,
    transactions: Vec<Transaction>,
    uncles: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    blockHash: Option<String>,
    blockNumber: Option<String>,
    from: String,
    gas: String,
    gasPrice: String,
    hash: String,
    input: String,
    nonce: String,
    to: Option<String>,
    transactionIndex: Option<String>,
    value: String,
    v: String,
    r: String,
    s: String,
}

lazy_static! {
    static ref ADDRESS_REGEX: Regex = Regex::new(r"0x[0-9A-Fa-f]{0,40}").unwrap();
    static ref BLOCKHASH_REGEX: Regex = Regex::new(r"0x[0-9A-Fa-f]{0,64}").unwrap();
}

impl Provider {
    pub fn new(_url: &str) -> Provider {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        Provider {
            url: _url.to_owned(),
            client: reqwest::blocking::Client::new(),
            headers: headers.clone(),
        }
    }

    pub fn block_number(&self) -> Result<u128, Box<dyn Error>> {
        let json: RPCResponse = self
            .client
            .post(&self.url)
            .body("{\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1,\"jsonrpc\":\"2.0\"}")
            .headers(self.headers.clone())
            .send()?
            .json()?;

        match json.error {
            Some(err) => Err(err.into()),
            None => Ok(u128::from_str_radix(
                json.result.unwrap().strip_prefix("0x").unwrap(),
                16,
            )?),
        }
    }

    pub fn gas_price(&self) -> Result<u128, Box<dyn Error>> {
        let json: RPCResponse = self
            .client
            .post(&self.url)
            .body("{\"method\":\"eth_gasPrice\",\"params\":[],\"id\":1,\"jsonrpc\":\"2.0\"}")
            .headers(self.headers.clone())
            .send()?
            .json()?;

        match json.error {
            Some(err) => Err(err.into()),
            None => Ok(u128::from_str_radix(
                json.result.unwrap().strip_prefix("0x").unwrap(),
                16,
            )?),
        }
    }

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
                    Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
                    Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
                    None => match block_number {
                        Some(block) => payload.push_str(&format!("0x{:x}", block)),
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
                    Some(err) => Err(err.into()),
                    None => Ok(u128::from_str_radix(
                        json.result.unwrap().strip_prefix("0x").unwrap(),
                        16,
                    )?),
                }
            }
            false => Err("Invalid address".into()),
        }
    }

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
                    Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
                    Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
                    None => match block_number {
                        Some(block) => payload.push_str(&format!("0x{:x}", block)),
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
                    Some(err) => Err(err.into()),
                    None => Ok(json.result.unwrap()),
                }
            }
            false => Err("Invalid address".into()),
        }
    }

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
                    Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
                    Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
                    None => match block_number {
                        Some(block) => payload.push_str(&format!("0x{:x}", block)),
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
                    Some(err) => Err(err.into()),
                    None => Ok(u128::from_str_radix(
                        json.result.unwrap().strip_prefix("0x").unwrap(),
                        16,
                    )?),
                }
            }
            false => Err("Invalid address".into()),
        }
    }

    pub fn get_block_by_hash(&self, block_hash: &str) -> Result<Option<Block>, Box<dyn Error>> {
        match BLOCKHASH_REGEX.is_match(block_hash) {
            true => {
                let mut payload = String::new();
                match write!(payload, "{{\"method\":\"eth_getBlockByHash\",\"params\":[\"{}\",false],\"id\":1,\"jsonrpc\":\"2.0\"}}", block_hash) {
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

    pub fn get_block_by_hash_with_tx(
        &self,
        block_hash: &str,
    ) -> Result<Option<BlockWithTx>, Box<dyn Error>> {
        match BLOCKHASH_REGEX.is_match(block_hash) {
            true => {
                let mut payload = String::new();
                match write!(payload, "{{\"method\":\"eth_getBlockByHash\",\"params\":[\"{}\",true],\"id\":1,\"jsonrpc\":\"2.0\"}}", block_hash) {
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

    pub fn get_block_by_number(
        &self,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<Option<Block>, Box<dyn Error>> {
        let mut payload = String::new();
        payload.push_str("{\"method\":\"eth_getBlockByNumber\",\"params\":[\"");
        match block_param {
            Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
            Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
            Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
            None => match block_number {
                Some(block) => payload.push_str(&format!("0x{:x}", block)),
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

    pub fn get_block_by_number_with_tx(
        &self,
        block_param: Option<DefaultBlockParam>,
        block_number: Option<u128>,
    ) -> Result<Option<BlockWithTx>, Box<dyn Error>> {
        let mut payload = String::new();
        payload.push_str("{\"method\":\"eth_getBlockByNumber\",\"params\":[\"");
        match block_param {
            Some(DefaultBlockParam::EARLIEST) => payload.push_str("earliest"),
            Some(DefaultBlockParam::LATEST) => payload.push_str("latest"),
            Some(DefaultBlockParam::PENDING) => payload.push_str("pending"),
            None => match block_number {
                Some(block) => payload.push_str(&format!("0x{:x}", block)),
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
}
