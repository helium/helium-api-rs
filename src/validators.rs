use crate::*;
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
