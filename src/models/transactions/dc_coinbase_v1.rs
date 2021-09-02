use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DcCoinbaseV1 {
    pub hash: String,
    pub payee: String,
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub amount: Hnt,
}
