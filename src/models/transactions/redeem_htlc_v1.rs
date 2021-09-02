use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RedeemHtlcV1 {
    pub fee: u64,
    pub hash: String,
    pub payee: String,
    pub address: String,
    pub preimage: String,
}
