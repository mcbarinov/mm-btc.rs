use std::time::Duration;

use rand::seq::SliceRandom;
use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};

use crate::util::str_contains;
use crate::Error;

const BASE_URL: &str = "https://blockstream.info/api";
const INVALID_ADDRESS_ERRORS: &[&str] = &["invalid bitcoin address", "bech32 segwit decoding error", "base58 error"];

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub funded_txo_count: u64,
    pub funded_txo_sum: u64,
    pub spent_txo_count: u64,
    pub spent_txo_sum: u64,
    pub tx_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(alias = "chain_stats")]
    pub chain: Stats,
    #[serde(alias = "mempool_stats")]
    pub mempool: Stats,
}

impl Address {
    pub fn confirmed_balance(&self) -> u64 {
        self.chain.funded_txo_sum - self.chain.spent_txo_sum
    }
}

pub async fn get_address(address: impl Into<String>, timeout: u64, proxy: Option<impl Into<String>>) -> Result<Address, Error> {
    let mut client = Client::builder().timeout(Duration::from_secs(timeout));
    if let Some(proxy) = proxy {
        client = client.proxy(Proxy::all(proxy.into())?);
    }

    let client = client.build()?;
    let res = client.get(format!("{BASE_URL}/address/{}", address.into())).send().await?;
    let status = res.status();
    let text = res.text().await?;

    if status == 400 && str_contains(&text.to_lowercase(), INVALID_ADDRESS_ERRORS) {
        return Err(Error::InvalidAddress);
    }
    let address: Address = serde_json::from_str(&text)?;
    Ok(address)
}

pub async fn get_address_with_attempts(
    address: String,
    timeout: u64,
    proxies: Vec<String>,
    attempts: u8,
) -> Result<Address, Error> {
    let mut result: Result<Address, Error> = Err(Error::Error);
    let mut rng = rand::thread_rng();
    for _ in 0..attempts {
        let proxy = proxies.choose(&mut rng);
        result = get_address(&address, timeout, proxy).await;

        match result {
            Ok(_) => return result,
            Err(Error::InvalidAddress) => return result,
            Err(_) => {
                continue;
            }
        }
    }
    result
}
