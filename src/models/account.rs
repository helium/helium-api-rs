use super::{Hnt, Hst, Iot, Mobile};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a wallet on the blockchain.
pub struct Account {
    /// The wallet address is the base58 check-encoded public key of
    /// the wallet.
    pub address: String,
    /// Block height of the API when query was made. When null, there
    /// is no on-chain record of this account.
    pub block: Option<u64>,
    /// The latest HNT balance of the wallet at block height
    pub balance: Hnt,
    /// The latest staked_balance of the wallet at block height
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub staked_balance: Hnt,
    /// The data credit balance of the wallet known at block height
    pub dc_balance: u64,
    /// The security token balance of the wallet at block height
    #[serde(deserialize_with = "Hst::deserialize")]
    pub sec_balance: Hst,
    /// The current nonce for the account
    pub nonce: u64,
    #[serde(default)]
    /// The latest IOT balance of the wallet at block height
    pub iot_balance: Iot,
    #[serde(default)]
    /// The latest MOB balance of the wallet at block height
    pub mobile_balance: Mobile,
    /// The current sec_nonce for the account
    pub sec_nonce: u64,
    /// The current dc_nonce for the account
    pub dc_nonce: u64,
    /// The speculative nonce for the account
    #[serde(default)]
    pub speculative_nonce: u64,
    /// The speculative security nonce for the account
    #[serde(default)]
    pub speculative_sec_nonce: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Role {
    #[serde(rename = "type")]
    pub role_type: String,
    pub time: u64,
    pub role: String,
    pub height: u64,
    pub hash: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RoleCount {
    pub vars_v1: Option<u64>,
    pub gen_validator_v1: Option<u64>,
    pub price_oracle_v1: Option<u64>,
    pub security_exchange_v1: Option<u64>,
    pub gen_gateway_v1: Option<u64>,
    pub consensus_group_v1: Option<u64>,
    pub token_burn_exchange_rate_v1: Option<u64>,
    pub transfer_hotspot_v2: Option<u64>,
    pub poc_receipts_v1: Option<u64>,
    pub validator_heartbeat_v1: Option<u64>,
    pub create_htlc_v1: Option<u64>,
    pub transfer_validator_stake_v1: Option<u64>,
    pub stake_validator_v1: Option<u64>,
    pub routing_v1: Option<u64>,
    pub poc_request_v1: Option<u64>,
    pub payment_v1: Option<u64>,
    pub assert_location_v2: Option<u64>,
    pub security_coinbase_v1: Option<u64>,
    pub assert_location_v1: Option<u64>,
    pub token_burn_v1: Option<u64>,
    pub rewards_v1: Option<u64>,
    pub unstake_validator_v1: Option<u64>,
    pub oui_v1: Option<u64>,
    pub state_channel_open_v1: Option<u64>,
    pub rewards_v2: Option<u64>,
    pub coinbase_v1: Option<u64>,
    pub add_gateway_v1: Option<u64>,
    pub poc_receipts_v2: Option<u64>,
    pub consensus_group_failure_v1: Option<u64>,
    pub payment_v2: Option<u64>,
    pub transfer_hotspot_v1: Option<u64>,
    pub dc_coinbase_v1: Option<u64>,
    pub state_channel_close_v1: Option<u64>,
    pub redeem_htlc_v1: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AccountReward {
    pub r#type: Option<String>,
    pub timestamp: String,
    pub hash: String,
    pub gateway: String,
    pub block: u64,
    pub amount: Hnt,
    pub account: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AccountRewardsTotals {
    pub total: f64,
    pub sum: Hnt,
    pub stddev: f64,
    pub min: f64,
    pub median: f64,
    pub max: f64,
    pub avg: f64,
    pub timestamp: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AccountStats {
    pub last_day: Vec<StatsMeasures>,
    pub last_month: Vec<StatsMeasures>,
    pub last_week: Vec<StatsMeasures>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StatsMeasures {
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub balance: Hnt,
    pub timestamp: String,
}
