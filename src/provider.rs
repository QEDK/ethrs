use lazy_static::lazy_static;
use primitive_types::U256;
use regex::Regex;
use reqwest;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::de::{self, Visitor};
use serde::Deserialize;
use serde::Deserializer;
use std::error::Error;
use std::fmt;
use std::fmt::Write;
use std::marker::PhantomData;
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
pub struct TxRPCResponse {
    error: Option<String>,
    result: Option<Transaction>,
}

#[derive(Deserialize, Debug)]
pub struct BlockWithTxRPCResponse {
    error: Option<String>,
    result: Option<BlockWithTx>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(deserialize_with = "option_string_as_u256")]
    number: Option<U256>,
    hash: Option<String>,
    parent_hash: String,
    #[serde(deserialize_with = "option_string_as_u256")]
    nonce: Option<U256>,
    sha3_uncles: String,
    logs_bloom: Option<String>,
    transactions_root: String,
    state_root: String,
    receipts_root: String,
    miner: Option<String>,
    #[serde(deserialize_with = "string_as_u256")]
    difficulty: U256,
    #[serde(deserialize_with = "option_string_as_u256")]
    total_difficulty: Option<U256>,
    extra_data: String,
    size: String,
    #[serde(deserialize_with = "string_as_u256")]
    gas_limit: U256,
    #[serde(deserialize_with = "string_as_u256")]
    gas_used: U256,
    timestamp: String,
    transactions: Vec<String>,
    uncles: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockWithTx {
    #[serde(deserialize_with = "option_string_as_u256")]
    number: Option<U256>,
    hash: Option<String>,
    parent_hash: String,
    #[serde(deserialize_with = "option_string_as_u256")]
    nonce: Option<U256>,
    sha3_uncles: String,
    logs_bloom: Option<String>,
    transactions_root: String,
    state_root: String,
    receipts_root: String,
    miner: Option<String>,
    #[serde(deserialize_with = "string_as_u256")]
    difficulty: U256,
    #[serde(deserialize_with = "option_string_as_u256")]
    total_difficulty: Option<U256>,
    extra_data: String,
    #[serde(deserialize_with = "string_as_u256")]
    size: U256,
    #[serde(deserialize_with = "string_as_u256")]
    gas_limit: U256,
    #[serde(deserialize_with = "string_as_u256")]
    gas_used: U256,
    timestamp: String,
    transactions: Vec<Transaction>,
    uncles: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    block_hash: Option<String>,
    #[serde(deserialize_with = "option_string_as_u256")]
    block_number: Option<U256>,
    from: String,
    #[serde(deserialize_with = "string_as_u256")]
    gas: U256,
    #[serde(deserialize_with = "string_as_u256")]
    gas_price: U256,
    hash: String,
    input: String,
    #[serde(deserialize_with = "string_as_u256")]
    nonce: U256,
    to: Option<String>,
    transaction_index: Option<String>,
    #[serde(deserialize_with = "string_as_u256")]
    value: U256,
    v: String,
    r: String,
    s: String,
}

fn string_as_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug)]
    struct U256String;

    impl<'de> de::Visitor<'de> for U256String {
        type Value = U256;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid hexstring < 2^256")
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(U256::from_str_radix(&value, 16).unwrap())
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(U256::from_str_radix(value, 16).unwrap())
        }
    }

    deserializer.deserialize_string(U256String)
}

fn option_string_as_u256<'de, D>(deserializer: D) -> Result<Option<U256>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug)]
    struct U256OptionString;

    impl<'de> de::Visitor<'de> for U256OptionString {
        type Value = Option<U256>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid option hexstring < 2^256")
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            string_as_u256(d).map(Some)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_option(U256OptionString)
}

lazy_static! {
    static ref ADDRESS_REGEX: Regex = Regex::new(r"0x[0-9A-Fa-f]{40}").unwrap();
    static ref BLOCKHASH_REGEX: Regex = Regex::new(r"0x[0-9A-Fa-f]{64}").unwrap();
    static ref SLOT_REGEX: Regex = Regex::new(r"0x[0-9A-Fa-f]{1,64}").unwrap();
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
                false => Err("Invalid slot".into()),
            },
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

    pub fn get_transaction_by_hash(
        &self,
        txhash: &str,
    ) -> Result<Option<Transaction>, Box<dyn Error>> {
        match BLOCKHASH_REGEX.is_match(txhash) {
            true => {
                let mut payload = String::new();
                match write!(payload, "{{\"method\":\"eth_getTransactionByHash\",\"params\":[\"{}\"],\"id\":1,\"jsonrpc\":\"2.0\"}}", txhash) {
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
}
