use super::Usd;
use serde::Deserialize;

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
