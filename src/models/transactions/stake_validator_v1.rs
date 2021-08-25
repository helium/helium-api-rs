use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StakeValidatorV1 {
    pub address: String,
    pub fee: u64,
    pub hash: String,
    pub owner: String,
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub stake: Hnt,
    pub owner_signature: String,
}
