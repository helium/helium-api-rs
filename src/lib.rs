#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod hnt;
pub use hnt::Hnt;

use helium_proto::{BlockchainTxn, Message, Txn};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// The default timeout for API requests
pub const DEFAULT_TIMEOUT: u64 = 120;
/// The default base URL if none is specified.
pub const DEFAULT_BASE_URL: &str = "https://api.helium.io/v1";

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
    pub sec_balance: u64,
    /// The current nonce for the account
    pub nonce: u64,
    /// The speculative nonce for the account
    #[serde(default)]
    pub speculative_nonce: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Geocode {
    /// The long version of city for the last asserted location
    pub long_city: Option<String>,
    /// The long version of country for the last asserted location
    pub long_country: Option<String>,
    /// The long version of state for the last asserted location
    pub long_state: Option<String>,
    /// The long version of street for the last asserted location
    pub long_street: Option<String>,
    /// The short version of city for the last asserted location
    pub short_city: Option<String>,
    /// The short version of country for the last asserted location
    pub short_country: Option<String>,
    /// The short version of state for the last asserted location
    pub short_state: Option<String>,
    /// The short version of street for the last asserted location
    pub short_street: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Height {
    /// The current block height of the chain.
    pub height: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Hotspot {
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
    pub lat: Option<f64>,
    /// The last asserted longitude of the hotspot
    pub lng: Option<f64>,
    /// The h3 index based on the lat/lon of the hotspot is used for
    /// PoC challenges.
    pub location: Option<String>, // h3
    /// The geocode information for the hotspot loocation
    pub geocode: Geocode,
    /// The current known score of the hotspos
    pub score: f32,
    /// The last block the score for the hotspot was updated. None if
    /// the score was never updated.
    pub score_update_height: Option<u64>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PendingTxnStatus {
    pub hash: String,
}

#[derive(Clone, Deserialize, Debug)]
pub(crate) struct Data<T> {
    pub data: T,
}

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Default for Client {
    /// Create a new client using the hosted Helium API at
    /// explorer.helium.foundation
    fn default() -> Self {
        Self::new_with_base_url(DEFAULT_BASE_URL.to_string())
    }
}

impl Client {
    /// Create a new client using a given base URL and a default
    /// timeout. The library will use absoluate paths based on this
    /// base_url.
    pub fn new_with_base_url(base_url: String) -> Self {
        Self::new_with_timeout(base_url, DEFAULT_TIMEOUT)
    }

    /// Create a new client using a given base URL, and request
    /// timeout value.  The library will use absoluate paths based on
    /// the given base_url.
    pub fn new_with_timeout(base_url: String, timeout: u64) -> Self {
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

    pub(crate) fn post<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        path: String,
        json: &T,
    ) -> Result<R> {
        let request_url = format!("{}{}", self.base_url, path);
        let mut response = self
            .client
            .post(&request_url)
            .json(json)
            .send()?
            .error_for_status()?;
        let result: Data<R> = response.json()?;
        Ok(result.data)
    }

    /// Get wallet information for a given address
    pub fn get_account(&self, address: &str) -> Result<Account> {
        self.fetch::<Account>(format!("/accounts/{}", address))
    }

    /// Get the current block height
    pub fn get_height(&self) -> Result<u64> {
        let result = self.fetch::<Height>("/blocks/height".to_string())?;
        Ok(result.height)
    }

    /// Get hotspots for a given wallet address
    pub fn get_hotspots(&self, address: &str) -> Result<Vec<Hotspot>> {
        self.fetch::<Vec<Hotspot>>(format!("/accounts/{}/hotspots", address))
    }

    /// Get details for a given hotspot address
    pub fn get_hotspot(&self, address: &str) -> Result<Hotspot> {
        self.fetch::<Hotspot>(format!("/hotspots/{}", address))
    }

    /// Convert a given transaction to json, ready to be submitted
    /// Submit a transaction to the blockchain
    pub fn submit_txn(&self, txn: Txn) -> Result<PendingTxnStatus> {
        let json = Client::txn_to_json(txn)?;
        self.post("/pending_transactions".to_string(), &json)
    }

    pub fn txn_to_b64(txn: Txn) -> Result<String> {
        let wrapper = BlockchainTxn { txn: Some(txn) };
        let mut buf = vec![];
        wrapper.encode(&mut buf)?;
        Ok(base64::encode(&buf))
    }

    pub fn txn_to_json(txn: Txn) -> Result<serde_json::Value> {
        Ok(json!({ "txn": Self::txn_to_b64(txn)?}))
    }
}
