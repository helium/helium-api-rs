use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TokenBurnExchangeRateV1 {
    pub hash: String,
    pub rate: u64,
}
