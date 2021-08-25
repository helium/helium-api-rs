use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PocRequestV1 {
    pub hash: String,
    pub block_hash: String,
    pub challenger: String,
    pub fee: u64,
    pub onion_key_hash: String,
    pub secret_hash: String,
    pub version: u64,
}
