#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod hnt;
pub use hnt::Hnt;

pub mod transactions;
use transactions::Transaction;

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

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents block metadata on the blockchain
pub struct Block {
    pub transactions_count: usize,
    pub time: usize,
    pub prev_hash: String,
    pub hash: String,
    pub height: usize,
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
    cursor: Option<String>,
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

#[derive(Clone, Deserialize, Debug)]
/// Represents an oracle price response from the API
pub struct OraclePrice {
    pub price: u64,
    pub block: u64,
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
        // peel away the cursor from fetch_with_cursor
        match self.fetch_with_cursor(path) {
            Ok((data, _cursor)) => Ok(data),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn fetch_with_cursor<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<(T, Option<String>)> {
        let request_url = format!("{}{}", self.base_url, path);
        let mut response = self.client.get(&request_url).send()?.error_for_status()?;
        let result: Data<T> = response.json()?;
        Ok((result.data, result.cursor))
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

    /// Get current oracle price
    pub fn get_oracle_price(&self) -> Result<OraclePrice> {
        self.fetch::<OraclePrice>("/oracle/prices/current")
    }

    /// Get oracle price at a specific block height
    pub fn get_oracle_price_at_height(&self, height: usize) -> Result<OraclePrice> {
        self.fetch::<OraclePrice>(&format!("/oracle/prices/{}", height))
    }

    /// Get transactions associated to a given account
    pub fn get_oracle_prices(&self) -> Result<(Vec<OraclePrice>, Option<String>)> {
        self.fetch_with_cursor::<Vec<OraclePrice>>("/oracle/prices")
    }

    /// Get more transactions associated to a given account
    pub fn get_more_oracle_prices(
        &self,
        cursor: &str,
    ) -> Result<(Vec<OraclePrice>, Option<String>)> {
        self.fetch_with_cursor::<Vec<OraclePrice>>(&format!("/oracle/prices?cursor={}", cursor))
    }

    /// Get wallet information for a given address
    pub fn get_account(&self, address: &str) -> Result<Account> {
        self.fetch::<Account>(&format!("/accounts/{}", address))
    }

    /// Get transactions associated to a given account
    pub fn get_account_transactions(
        &self,
        address: &str,
    ) -> Result<(Option<Vec<Transaction>>, Option<String>)> {
        let (transactions, cursor) =
            self.fetch_with_cursor::<Vec<Transaction>>(&format!("/accounts/{}/activity", address))?;

        // For some reason, the first fetch sometimes returns a cursor and nothing else
        if transactions.is_empty() && cursor.is_some() {
            if let Some(cursor) = cursor {
                self.get_more_account_transactions(address, cursor.as_str())
            } else {
                Ok((None, cursor))
            }
        } else {
            Ok((Some(transactions), cursor))
        }
    }

    /// Get more transactions associated to a given account
    pub fn get_more_account_transactions(
        &self,
        address: &str,
        cursor: &str,
    ) -> Result<(Option<Vec<Transaction>>, Option<String>)> {
        let (transactions, cursor) = self.fetch_with_cursor::<Vec<Transaction>>(&format!(
            "/accounts/{}/activity?cursor={}",
            address, cursor
        ))?;
        if transactions.is_empty() {
            Ok((None, cursor))
        } else {
            Ok((Some(transactions), cursor))
        }
    }

    /// Get a specific transaction by hash
    pub fn get_transaction(&self, hash: &str) -> Result<Transaction> {
        self.fetch::<Transaction>(&format!("/transactions/{}", hash))
    }

    /// Get metadata transactions from a given block
    pub fn get_block(&self, block: u64) -> Result<Block> {
        self.fetch::<Block>(&format!("/blocks/{}", block))
    }

    /// Get transactions from a given block
    pub fn get_block_transactions(&self, block: u64) -> Result<(Vec<Transaction>, Option<String>)> {
        self.fetch_with_cursor::<Vec<Transaction>>(&format!("/blocks/{}/transactions", block))
    }

    /// Get additional transactions from the block using cursor
    pub fn get_more_block_transactions(
        &self,
        block: u64,
        cursor: &str,
    ) -> Result<(Vec<Transaction>, Option<String>)> {
        self.fetch_with_cursor::<Vec<transactions::Transaction>>(&format!(
            "/blocks/{}/transactions?cursor={}",
            block, cursor
        ))
    }

    /// Get the current block height
    pub fn get_height(&self) -> Result<u64> {
        let result = self.fetch::<Height>("/blocks/height")?;
        Ok(result.height)
    }

    /// Get list of all hotspots
    pub fn get_hotspots(&self) -> Result<(Vec<Hotspot>, Option<String>)> {
        self.fetch_with_cursor::<Vec<Hotspot>>("/hotspots")
    }

    /// Get additional hotspots using cursor
    pub fn get_more_hotspots(&self, cursor: &str) -> Result<(Vec<Hotspot>, Option<String>)> {
        self.fetch_with_cursor::<Vec<Hotspot>>(&format!("/hotspots?cursor={}", cursor))
    }

    /// Get hotspots for a given wallet address
    pub fn get_account_hotspots(&self, address: &str) -> Result<Vec<Hotspot>> {
        self.fetch::<Vec<Hotspot>>(&format!("/accounts/{}/hotspots", address))
    }

    /// Get details for a given hotspot address
    pub fn get_hotspot(&self, address: &str) -> Result<Hotspot> {
        self.fetch::<Hotspot>(&format!("/hotspots/{}", address))
    }

    /// Get activity for a given hotspot address
    pub fn get_hotspot_activity(
        &self,
        address: &str,
    ) -> Result<(Vec<Transaction>, Option<String>)> {
        self.fetch_with_cursor::<Vec<Transaction>>(&format!("/hotspots/{}/activity", address))
    }

    /// Get activity for a given hotspot address
    pub fn get_more_hotspot_activity(
        &self,
        address: &str,
        cursor: &str,
    ) -> Result<(Vec<Transaction>, Option<String>)> {
        self.fetch_with_cursor::<Vec<Transaction>>(&format!(
            "/hotspots/{}/activity?cursor={}",
            address, cursor
        ))
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
