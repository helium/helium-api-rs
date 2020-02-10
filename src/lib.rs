#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use helium_proto::{BlockchainTxn, Txn};
use helium_proto::Message;
use reqwest;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// The default timeout for API requests
pub const DEFAULT_TIMEOUT: u64 = 120;
/// The default base URL if none is specified.
pub const DEFAULT_BASE_URL: &str = "https://explorer.helium.foundation/api";

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Deserialize, Debug)]
/// Represents a wallet on the blockchain.
pub struct Account {
    /// The wallet address is the base58 check-encoded public key of
    /// the wallet.
    pub address: String,
    /// The latest balance of the wallet known to the API
    pub balance: u64,
    /// The data credit balance of the wallet known to the API
    pub dc_balance: u64,
    /// The security token balance of the wallet known to the API
    pub security_balance: u64,
    /// The current nonce for the account
    pub nonce: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Hotspot {
    #[serde(alias = "gateway", alias = "address")]
    /// The address of the hotspots. This is the public key in base58
    /// check-encoding of the hotspot.
    pub address: String,
    /// The hotspot owner wallet address
    pub owner: String,
    /// The "animal" name of the hotspot. The name can be `None` for
    /// some API endpoints.
    pub name: Option<String>,
    /// The block height when the hotspot was added to the blockchain
    pub added_height: Option<u64>,
    /// The last asserted latitude of the hotspot
    pub lat: f64,
    /// The last asserted longitude of the hotspot
    pub lng: f64,
    /// The h3 index based on the lat/lon of the hotspot is used for
    /// PoC challenges.
    pub location: String, // h3
    /// The long version of city for the last asserted location
    pub long_city: String,
    /// The long version of country for the last asserted location
    pub long_country: String,
    /// The long version of state for the last asserted location
    pub long_state: String,
    /// The long version of street for the last asserted location
    pub long_street: String,
    /// The short version of city for the last asserted location
    pub short_city: String,
    /// The short version of country for the last asserted location
    pub short_country: String,
    /// The short version of state for the last asserted location
    pub short_state: String,
    /// The short version of street for the last asserted location
    pub short_street: String,
    /// The current known score of the hotspos
    pub score: f32,
    /// The last block the score for the hotspot was updated. None if
    /// the score was never updated.
    pub score_update_height: Option<u64>,
}

#[derive(Clone, Deserialize, Debug)]
pub(crate) struct Data<T> {
    pub data: T,
}

#[derive(Clone, Debug)]
pub struct Client {
    base_url: &'static str,
    client: reqwest::Client,
}

impl Default for Client {
    /// Create a new client using the hosted Helium API at
    /// explorer.helium.foundation
    fn default() -> Self {
        Self::new_with_base_url(DEFAULT_BASE_URL)
    }
}

impl Client {
    /// Create a new client using a given base URL and a default
    /// timeout. The library will use absoluate paths based on this
    /// base_url.
    pub fn new_with_base_url(base_url: &'static str) -> Self {
        Self::new_with_timeout(base_url, DEFAULT_TIMEOUT)
    }

    /// Create a new client using a given base URL, and request
    /// timeout value.  The library will use absoluate paths based on
    /// the given base_url.
    pub fn new_with_timeout(base_url: &'static str, timeout: u64) -> Self {
        let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap();
        Self { base_url, client }
    }

    pub(crate) fn fetch<T: DeserializeOwned>(&self, path: String) -> Result<T> {
        let request_url = format!("{}{}", self.base_url, path);
        let mut response = self.client.get(&request_url).send()?.error_for_status()?;
        let result: Data<T> = response.json()?;
        Ok(result.data)
    }

    pub(crate) fn post<T: Serialize + ?Sized>(&self, path: String, json: &T) -> Result {
        let request_url = format!("{}{}", self.base_url, path);
        let response = self.client.post(&request_url).json(json).send()?;
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }

    /// Get wallet information for a given address
    pub fn get_account(&self, address: &str) -> Result<Account> {
        self.fetch::<Account>(format!("/accounts/{}", address))
    }

    /// Get hotspots for a given wallet address
    pub fn get_hotspots(&self, address: &str) -> Result<Vec<Hotspot>> {
        self.fetch::<Vec<Hotspot>>(format!("/accounts/{}/gateways", address))
    }

    /// Get details for a given hotspot address
    pub fn get_hotspot(&self, address: &str) -> Result<Hotspot> {
        self.fetch::<Hotspot>(format!("/hotspots/{}", address))
    }

    /// Submit a transaction to the blockchain
    pub fn submit_txn(&self, txn: Txn) -> Result {
        let wrapper = BlockchainTxn { txn: Some(txn) };
        let mut buf = vec![];
        wrapper.encode(&mut buf)?;
        self.post(
            "/transactions".to_string(),
            &json!({ "txn": base64::encode(&buf) }),
        )
    }
}
