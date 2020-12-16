#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod hnt;
pub use hnt::Hnt;
mod hst;
pub use hst::Hst;

pub use helium_proto::*;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// The default timeout for API requests
pub const DEFAULT_TIMEOUT: u64 = 120;
/// The default base URL if none is specified.
pub const DEFAULT_BASE_URL: &str = "https://api.helium.io/v1";

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Serialize, Deserialize, Debug)]
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
    /// The speculative security nonce for the account
    #[serde(default)]
    pub speculative_sec_nonce: u64,
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

    pub(crate) fn fetch<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let request_url = format!("{}{}", self.base_url, path);
        let mut response = self.client.get(&request_url).send()?.error_for_status()?;
        let result: Data<T> = response.json()?;
        Ok(result.data)
    }

    pub(crate) fn post<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        path: &str,
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
        self.fetch::<Account>(&format!("/accounts/{}", address))
    }

    /// Get the current block height
    pub fn get_height(&self) -> Result<u64> {
        let result = self.fetch::<Height>("/blocks/height")?;
        Ok(result.height)
    }

    /// Get hotspots for a given wallet address
    pub fn get_hotspots(&self, address: &str) -> Result<Vec<Hotspot>> {
        self.fetch::<Vec<Hotspot>>(&format!("/accounts/{}/hotspots", address))
    }

    /// Get details for a given hotspot address
    pub fn get_hotspot(&self, address: &str) -> Result<Hotspot> {
        self.fetch::<Hotspot>(&format!("/hotspots/{}", address))
    }

    /// Get the current active set of chain variables
    pub fn get_vars(&self) -> Result<serde_json::Map<String, serde_json::Value>> {
        let result = self.fetch::<serde_json::Value>("/vars")?;
        match result.as_object() {
            Some(map) => Ok(map.clone()),
            None => Err("Expected a chain variable map".into()),
        }
    }

    /// Get the last assigned OUI value
    pub fn get_last_oui(&self) -> Result<u64> {
        let result = self.fetch::<serde_json::Value>("/ouis/last")?;
        match result["oui"].as_u64() {
            Some(oui) => Ok(oui),
            None => Err("Expected an oui field".into()),
        }
    }

    /// Convert a given transaction to json, ready to be submitted
    /// Submit a transaction to the blockchain
    pub fn submit_txn(&self, txn: &BlockchainTxn) -> Result<PendingTxnStatus> {
        let json = Client::txn_to_json(txn)?;
        self.post("/pending_transactions", &json)
    }

    /// Convert a given transaction to it's b64 encoded binary
    /// form. The encoded transaction is ready for submission to the
    /// api
    pub fn txn_to_b64(txn: &BlockchainTxn) -> Result<String> {
        let mut buf = vec![];
        txn.encode(&mut buf)?;
        Ok(base64::encode(&buf))
    }

    /// Convert the given transaction to the json that is required to
    /// be submitted to the api endpoint
    pub fn txn_to_json(txn: &BlockchainTxn) -> Result<serde_json::Value> {
        Ok(json!({ "txn": Self::txn_to_b64(txn)?}))
    }
}
