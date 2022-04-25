use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TransferHotspotV1 {
    pub hash: String,
    pub fee: u64,
    pub buyer: String,
    pub seller: String,
    pub gateway: String,
    pub buyer_nonce: u64,
    pub amount_to_seller: Hnt,
}
