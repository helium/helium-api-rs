use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OuiV1 {
    pub fee: u64,
    pub oui: u64,
    pub hash: String,
    pub owner: String,
    pub payer: String,
    pub filter: String,
    pub addresses: Vec<String>,
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub staking_fee: Hnt,
    pub requested_subnet_size: u64,
}
