use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaymentV2 {
    pub hash: String,
    /// Fee is in datacredits
    pub fee: u64,
    pub nonce: u64,
    pub payer: String,
    pub payments: Vec<PaymentV2Payment>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaymentV2Payment {
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub amount: Hnt,
    pub memo: Option<String>,
    pub payee: String,
}
