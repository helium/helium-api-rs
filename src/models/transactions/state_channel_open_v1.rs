use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StateChannelOpenV1 {
    pub id: String,
    pub fee: u64,
    pub oui: u64,
    pub hash: String,
    pub nonce: u64,
    pub owner: String,
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub amount: Hnt,
    pub expire_within: u64,
}
