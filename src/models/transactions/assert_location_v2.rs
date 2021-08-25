use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AssertLocationV2 {
    pub hash: String,
    pub fee: u64,
    pub gain: i64,
    pub nonce: u64,
    pub owner: String,
    pub payer: Option<String>,
    pub gateway: String,
    pub location: String,
    pub elevation: i64,
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub staking_fee: Hnt,
}
