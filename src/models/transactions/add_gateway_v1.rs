use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AddGatewayV1 {
    pub hash: String,
    pub fee: u64,
    pub owner: String,
    pub payer: String,
    pub gateway: String,
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub staking_fee: Hnt,
}
