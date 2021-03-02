use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

#[macro_use]
pub(crate) mod bones_macro;

mod hnt;
pub use hnt::Hnt;
mod hst;
pub use hst::Hst;
pub mod error;
mod oracle_price;
pub use oracle_price::OraclePrice;

pub use helium_proto::*;
use std::time::Duration;

/// The default timeout for API requests
pub const DEFAULT_TIMEOUT: u64 = 120;
/// The default base URL if none is specified.
pub const DEFAULT_BASE_URL: &str = "https://api.helium.io/v1";

#[derive(Clone, Deserialize, Debug, Serialize)]
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

#[derive(Clone, Deserialize, Debug, Serialize)]
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
    /// The geocode information for the hotspot location
    pub geocode: Geocode,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PendingTxnStatus {
    pub hash: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a validator on the blockchain.
pub struct Validator {
    /// The validator address is the base58 check-encoded public key of
    /// the validator.
    pub address: String,
    /// The validator pwner is the base58 check-encoded public key of
    /// the owner of the validator.
    pub owner: String,
    /// The staked amount for the validator
    pub stake: u64,
    /// The last heartbeat transaction of the validator
    pub last_heartbeat: u64,
    /// The last heartbeat version of the validator heartbeat
    pub version_heartbeat: u64,
    /// The current status of the validator (staked, cooldown, unstaked)
    pub status: String,
}

#[derive(Clone, Deserialize, Debug)]
pub(crate) struct Data<T> {
    pub data: T,
}

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    client: reqwest::blocking::Client,
}

#[derive(Clone, Deserialize, Debug)]
pub struct OraclePredictions {
    #[serde(with = "crate::oracle_price::deserializer")]
    pub price: OraclePrice,
    pub time: usize,
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
        let client = reqwest::blocking::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap();
        Self { base_url, client }
    }

    pub(crate) fn fetch<T: DeserializeOwned>(&self, path: &str) -> error::Result<T> {
        let request_url = format!("{}{}", self.base_url, path);
        let response = self.client.get(&request_url).send()?.error_for_status()?;
        let result: Data<T> = response.json()?;
        Ok(result.data)
    }

    pub(crate) fn post<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        path: &str,
        json: &T,
    ) -> error::Result<R> {
        let request_url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .post(&request_url)
            .json(json)
            .send()?
            .error_for_status()?;
        let result: Data<R> = response.json()?;
        Ok(result.data)
    }

    /// Get wallet information for a given address
    pub fn get_account(&self, address: &str) -> error::Result<Account> {
        self.fetch::<Account>(&format!("/accounts/{}", address))
    }

    /// Get the current block height
    pub fn get_height(&self) -> error::Result<u64> {
        let result = self.fetch::<Height>("/blocks/height")?;
        Ok(result.height)
    }

    /// Get hotspots for a given wallet address
    pub fn get_hotspots(&self, address: &str) -> error::Result<Vec<Hotspot>> {
        self.fetch::<Vec<Hotspot>>(&format!("/accounts/{}/hotspots", address))
    }

    /// Get details for a given hotspot address
    pub fn get_hotspot(&self, address: &str) -> error::Result<Hotspot> {
        self.fetch::<Hotspot>(&format!("/hotspots/{}", address))
    }

    /// Get validator information for a given address
    pub fn get_validator(&self, address: &str) -> error::Result<Validator> {
        self.fetch::<Validator>(&format!("/validators/{}", address))
    }

    /// Get the current active set of chain variables
    pub fn get_vars(&self) -> error::Result<serde_json::Map<String, serde_json::Value>> {
        let result = self.fetch::<serde_json::Value>("/vars")?;
        result
            .as_object()
            .cloned()
            .ok_or_else(|| error::value(result))
    }

    /// Get the last assigned OUI value
    pub fn get_last_oui(&self) -> error::Result<u64> {
        let result = self.fetch::<serde_json::Value>("/ouis/last")?;
        let oui = result["oui"].clone();
        oui.as_u64().ok_or_else(|| error::value(oui))
    }

    /// Get current oracle price
    pub fn get_oracle_price_current(&self) -> error::Result<OraclePrice> {
        #[derive(Clone, Deserialize, Debug)]
        struct Response {
            price: u64,
        }
        let response = self.fetch::<Response>("/oracle/prices/current")?;
        Ok(OraclePrice::from_bones(response.price))
    }

    /// Get current oracle price
    pub fn get_oracle_price_predicted(&self) -> error::Result<Vec<OraclePredictions>> {
        let response = self.fetch::<Vec<OraclePredictions>>("/oracle/predictions")?;
        Ok(response)
    }

    /// Convert a given transaction to json, ready to be submitted
    /// Submit a transaction to the blockchain
    pub fn submit_txn(&self, txn: &BlockchainTxn) -> error::Result<PendingTxnStatus> {
        let json = Client::txn_to_json(txn)?;
        self.post("/pending_transactions", &json)
    }

    /// Convert a given transaction to it's b64 encoded binary
    /// form. The encoded transaction is ready for submission to the
    /// api
    pub fn txn_to_b64(txn: &BlockchainTxn) -> error::Result<String> {
        let mut buf = vec![];
        txn.encode(&mut buf)?;
        Ok(base64::encode(&buf))
    }

    /// Convert the given transaction to the json that is required to
    /// be submitted to the api endpoint
    pub fn txn_to_json(txn: &BlockchainTxn) -> error::Result<serde_json::Value> {
        Ok(json!({ "txn": Self::txn_to_b64(txn)?}))
    }
}
