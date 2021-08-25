use crate::models::Usd;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PriceOracleV1 {
    pub fee: u64,
    pub hash: String,
    #[serde(deserialize_with = "Usd::deserialize")]
    pub price: Usd,
    pub public_key: String,
    pub block_height: u64,
}
