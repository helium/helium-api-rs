use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod account;
mod block;
mod geocode;
mod hotspot;
mod oracle;
mod oui;
pub mod transactions;
mod validator;
mod values;

pub use account::*;
pub use block::*;
pub use geocode::*;
pub use hotspot::*;
pub use oracle::*;
pub use oui::*;
pub use validator::*;
pub use values::*;

/// Query params for requests that can take in a time range.
#[derive(Clone, Debug, Serialize)]
pub struct QueryTimeRange {
    /// ISO 8601 timestamp or relative time (-3 hour) minimum time range
    pub min_time: String,
    /// ISO 8601 timestamp or relative time (-3 hour) maximum time range
    pub max_time: String,
}

/// Reward for validator or  hotspot
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reward {
    /// The owner address is the base58 check-encoded public key of
    /// the owner's wallet address.
    pub account: String,
    /// The reward amount.
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub amount: Hnt,
    /// The block the reward was earned in.
    pub block: u64,
    /// The validator or hotspot address is the base58 check-encoded public
    /// key of the validator or hotspot.
    pub gateway: String,
    /// The transaction hash of the reward.
    pub hash: String,
    /// The timestamp of the rewards.
    pub timestamp: DateTime<Utc>,
}
