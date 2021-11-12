use super::{Hnt, Hst};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a wallet on the blockchain.
pub struct Account {
    /// The wallet address is the base58 check-encoded public key of
    /// the wallet.
    pub address: String,
    /// The latest balance of the wallet known to the API
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub balance: Hnt,
    /// The latest staked balance of the wallet known to the API
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub staked_balance: Hnt,
    /// The current nonce for the account
    pub nonce: u64,
    /// The speculative nonce for the account
    #[serde(default)]
    pub speculative_nonce: u64,
    /// The data credit balance of the wallet known to the API
    pub dc_balance: u64,
    /// The current data credit nonce for the account
    pub dc_nonce: u64,
    /// The security token balance of the wallet known to the API
    #[serde(deserialize_with = "Hst::deserialize")]
    pub sec_balance: Hst,
    /// The current security nonce for the account
    pub sec_nonce: u64,
    /// The speculative security nonce for the account
    #[serde(default)]
    pub speculative_sec_nonce: u64,
}
