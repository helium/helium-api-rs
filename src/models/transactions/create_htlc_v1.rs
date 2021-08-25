use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateHtlcV1 {
    pub fee: u64,
    pub hash: String,
    pub nonce: u64,
    pub payee: String,
    pub payer: String,
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub amount: Hnt,
    pub address: String,
    pub hashlock: String,
    pub timelock: u64,
}
