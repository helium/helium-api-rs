use super::{Hnt, Hst};

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
    /// The latest balance of the wallet at block height
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
