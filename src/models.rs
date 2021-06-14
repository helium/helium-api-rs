use crate::values::{Dbi, Hnt, Hst, Usd};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a wallet on the blockchain.
pub struct Account {
    /// The wallet address is the base58 check-encoded public key of
    /// the wallet.
    pub address: String,
    /// The latest balance of the wallet known to the API
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub balance: Hnt,
    /// The data credit balance of the wallet known to the API
    pub dc_balance: u64,
    /// The security token balance of the wallet known to the API
    #[serde(deserialize_with = "Hst::deserialize")]
    pub sec_balance: Hst,
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

#[derive(Deserialize)]
pub struct Height {
    /// The current block height of the chain.
    pub height: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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
    pub location: Option<String>,
    /// The elevation (in meters) above or belowo sea level
    pub elevation: Option<i32>,
    /// The gain (in dbi) above or belowo sea level
    pub gain: Option<Dbi>,
    /// The geocode information for the hotspot location
    pub geocode: Geocode,
    /// The current nonce for the hotspot
    pub nonce: u64,
    /// The speculative nonce for the hotspot. This field is only meaningful
    /// when a single hotspot is requested
    #[serde(default)]
    pub speculative_nonce: u64,
}

/// An oracle prediction is inferred from the current oracle price and submitted
/// oracle price reports.
#[derive(Clone, Deserialize, Debug)]
pub struct OraclePrediction {
    /// The oracle price at the indicated block in Usd millis
    #[serde(deserialize_with = "Usd::deserialize")]
    pub price: Usd,
    /// The epoch time when the price is expected to take hold
    pub time: u64,
}

/// An oracle price is inferred from oracle price reports on a regular basis by
/// the blockchain. It determines the conversion rate between Hnt and Data
/// Credits.
#[derive(Clone, Deserialize, Debug)]
pub struct OraclePrice {
    /// The oracle price at the indicated block in Usd millis
    #[serde(deserialize_with = "Usd::deserialize")]
    pub price: Usd,
    /// The block height the oracle price was set at
    pub block: u64,
}

#[derive(Clone, Deserialize, Debug)]
/// Represents an OUI on the blockchain
pub struct Oui {
    /// The oui value.
    pub oui: u64,
    /// The base58 public key of the owner of the oui.
    pub owner: String,
    /// The current nonce for the oui
    pub nonce: u64,
    /// The base58 encoded public keys of the routers for this oui
    pub addresses: Vec<String>,
    /// The subnets for this oui
    pub subnets: Vec<Subnet>,
}

/// Stats for ouis
#[derive(Clone, Deserialize, Debug)]
pub struct OuiStats {
    pub count: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PendingTxnStatus {
    pub hash: String,
}

/// Query params for requests that can take in a time range.
#[derive(Clone, Debug, Serialize)]
pub struct QueryTimeRange {
    /// ISO 8601 timestamp or relative time (-3 hour) minimum time range
    pub min_time: String,
    /// ISO 8601 timestamp or relative time (-3 hour) maximum time range
    pub max_time: String,
}

/// Reward for validator
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reward {
    /// The owner address is the base58 check-encoded public key of
    /// the owner's wallet address.
    pub account: String,
    /// The reward amount.
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub amount: Hnt,
    /// The block the reward was earned in.
    pub block: i64,
    /// The validator address is the base58 check-encoded public key of
    /// the validator.
    pub gateway: String,
    /// The transaction hash of the reward.
    pub hash: String,
    /// The timestamp of the rewards.
    pub timestamp: DateTime<Utc>,
}

/// Stats for a specific validator stake status
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StakeStats {
    /// The amount of HNT committed in the staked status
    pub amount: Hnt,
    /// The number of validators in the staked status
    pub count: u64,
}

#[derive(Clone, Deserialize, Debug)]
/// An OUI owns a list of subnets, which are used to check if packets from a
/// device with a given DevAddr need to be sent to the routers in the OUI
pub struct Subnet {
    base: u32,
    mask: u32,
}

impl fmt::Display for Subnet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.base, self.mask))
    }
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
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub stake: Hnt,
    /// The last heartbeat transaction of the validator
    pub last_heartbeat: u64,
    /// The last heartbeat version of the validator heartbeat
    pub version_heartbeat: u64,
    /// The current status of the validator (staked, cooldown, unstaked)
    pub stake_status: String,
}

/// Stats for validators
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ValidatorStats {
    /// The number of active validators. An active validator is one that emits
    /// heartbeat transactions on a regular basis. `None` indicates the active
    /// number is unknown at this time.
    pub active: Option<u64>,
    pub staked: StakeStats,
    pub unstaked: StakeStats,
    pub cooldown: StakeStats,
}
