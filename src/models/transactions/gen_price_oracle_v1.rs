use crate::models::Usd;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GenPriceOracleV1 {
    pub hash: String,
    #[serde(deserialize_with = "Usd::deserialize")]
    pub price: Usd,
}
