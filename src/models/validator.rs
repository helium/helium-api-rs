use super::Hnt;
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
    pub stake: Hnt,
    /// The last heartbeat transaction of the validator
    pub last_heartbeat: u64,
    /// The last heartbeat version of the validator heartbeat
    pub version_heartbeat: u64,
    /// The current status of the validator (staked, cooldown, unstaked)
    pub stake_status: String,
    /// The total penalty of the validator
    pub penalty: f64,
    /// A list of penalties this validator has received
    pub penalties: Vec<Penalty>,
    /// The block this validator was added to chain
    pub block_added: u64,
    /// The current block this validator is synced to
    pub block: u64,
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
    pub amount: f64,
    /// The number of validators in the staked status
    pub count: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
/// The Penalty types reported for a validator.
pub enum PenaltyType {
    Performance,
    Tenure,
    Dkg,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a penalty for a validator.
pub struct Penalty {
    /// The type of penalty
    #[serde(rename = "type")]
    pub kind: PenaltyType,
    /// The block the penalty occured in.
    pub height: u64,
    /// The amount of penalty.
    pub amount: f64,
}
