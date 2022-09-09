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
    pub staked_balance: Hnt,
    /// The data credit balance of the wallet known at block height
    pub dc_balance: u64,
    /// The security token balance of the wallet at block height
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
/// Account as deserialized from ETL. Token types are from u64
pub(crate) struct AccountEtl {
    pub address: String,
    pub block: Option<u64>,
    #[serde(deserialize_with = "Hnt::deserialize_u64")]
    pub balance: Hnt,
    #[serde(deserialize_with = "Hnt::deserialize_u64")]
    pub staked_balance: Hnt,
    pub dc_balance: u64,
    #[serde(deserialize_with = "Hst::deserialize_u64")]
    pub sec_balance: Hst,
    pub nonce: u64,
    #[serde(default)]
    #[serde(deserialize_with = "Iot::deserialize_u64")]
    pub iot_balance: Iot,
    #[serde(default)]
    #[serde(deserialize_with = "Mobile::deserialize_u64")]
    pub mobile_balance: Mobile,
    pub sec_nonce: u64,
    pub dc_nonce: u64,
    #[serde(default)]
    pub speculative_nonce: u64,
    #[serde(default)]
    pub speculative_sec_nonce: u64,
}

impl From<AccountEtl> for Account {
    fn from(account_etl: AccountEtl) -> Self {
        Self {
            address: account_etl.address,
            block: account_etl.block,
            balance: account_etl.balance,
            staked_balance: account_etl.staked_balance,
            dc_balance: account_etl.dc_balance,
            sec_balance: account_etl.sec_balance,
            nonce: account_etl.nonce,
            iot_balance: account_etl.iot_balance,
            mobile_balance: account_etl.mobile_balance,
            sec_nonce: account_etl.sec_nonce,
            dc_nonce: account_etl.dc_nonce,
            speculative_nonce: account_etl.speculative_nonce,
            speculative_sec_nonce: account_etl.speculative_sec_nonce,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account() {
        let expected_hnt = Hnt::from(91422941);
        let etl_json = "\
        {
            \"validator_count\": 0,
            \"staked_balance\": 0,
            \"speculative_sec_nonce\": 0,
            \"speculative_nonce\": 46,
            \"sec_nonce\": 0,
            \"sec_balance\": 0,
            \"nonce\": 46,
            \"mobile_balance\": 0,
            \"iot_balance\": 0,
            \"hotspot_count\": 5,
            \"dc_nonce\": 0,
            \"dc_balance\": 0,
            \"block\": 1518967,
            \"balance\": 91422941,
            \"address\": \"13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH\"
        }";
        let account_etl: AccountEtl = serde_json::from_str(&etl_json).unwrap();
        assert_eq!(account_etl.balance, expected_hnt);
        let account: Account = account_etl.into();
        assert_eq!(account.balance, expected_hnt);
        let account_json = serde_json::to_string(&account).unwrap();
        let expected_json = "\
            {\"address\":\"13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH\",\
            \"block\":1518967,\"balance\":0.91422941,\"staked_balance\":0.0,\"dc_balance\":0,\
            \"sec_balance\":0.0,\"nonce\":46,\"iot_balance\":0.0,\"mobile_balance\":0.0,\
            \"sec_nonce\":0,\"dc_nonce\":0,\"speculative_nonce\":46,\"speculative_sec_nonce\":0}";
        assert_eq!(expected_json, account_json);
    }
}
