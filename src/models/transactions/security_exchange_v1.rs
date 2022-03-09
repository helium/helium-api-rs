use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SecurityExchangeV1 {
    pub fee: u64,
    pub hash: String,
    pub nonce: u64,
    pub payee: String,
    pub payer: String,
    pub amount: Hnt,
}
