use lazy_static::lazy_static;
use regex::Regex;
use reqwest;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Deserialize;
use std::error::Error;
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
struct RPCResponse {
    error: Option<String>,
    result: Option<String>,
}

lazy_static! {
    static ref ADDRESS_REGEX: Regex = Regex::new(r"0x[0-9A-Fa-f]{0,40}").unwrap();
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
}
