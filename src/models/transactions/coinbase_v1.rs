use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CoinbaseV1 {
    pub hash: String,
    pub payee: String,
    pub amount: Hnt,
}
