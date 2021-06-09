use crate::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub stake_status: String,
}

/// Stats for validators
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Stats {
    /// The number of active validators. An active validator is one that emits
    /// heartbeat transactions on a regular basis. `None` indicates the active
    /// number is unknown at this time.
    pub active: Option<u64>,
    pub staked: StakeStats,
    pub unstaked: StakeStats,
    pub cooldown: StakeStats,
}

/// Stats for a specific validator stake status
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StakeStats {
    /// The amount of HNT committed in the staked status
    pub amount: Hnt,
    /// The number of validators in the staked status
    pub count: u64,
}

/// Reward for validator
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reward {
    /// The owner address is the base58 check-encoded public key of
    /// the owner's wallet address.
    pub account: String,
    /// The reward amount.
    pub amount: i64,
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

/// Query params for requests that can take in a time range.
#[derive(Clone, Debug, Serialize)]
pub struct QueryTimeRange {
    /// ISO 8601 timestamp or relative time (-3 hour) minimum time range
    pub min_time: String,
    /// ISO 8601 timestamp or relative time (-3 hour) maximum time range
    pub max_time: String,
}

/// Get all known validators
pub fn all(client: &Client) -> Stream<Validator> {
    client.fetch_stream("/validators", NO_QUERY)
}

/// Get a specific validator
pub async fn get(client: &Client, address: &str) -> Result<Validator> {
    client
        .fetch(&format!("/validators/{}", address), NO_QUERY)
        .await
}

/// Get stats for validators
pub async fn stats(client: &Client) -> Result<Stats> {
    client.fetch("/validators/stats", NO_QUERY).await
}

/// Get rewards for a validator
///
/// Returns rewards for a given validator per reward block the validator is in,
/// for a given timeframe. `QueryTimeRange` contains the timestamps given in 
/// 4ISO 8601 format, or in relative time. The block that contains the max_time
/// timestamp is excluded from the result.
pub async fn rewards(client: &Client, address: &str, query: &QueryTimeRange) -> Stream<Reward> {
    client.fetch_stream(&format!("/validators/{}/rewards", address), query)
}
