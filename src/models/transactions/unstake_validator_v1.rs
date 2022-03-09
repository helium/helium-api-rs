use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UnstakeValidatorV1 {
    pub address: String,
    pub owner: String,
    pub owner_signature: String,
    pub fee: u64,
    pub stake_amount: Hnt,
    pub stake_release_height: u64,
    pub hash: String,
}
