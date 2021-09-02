use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ValidatorHeartbeatV1 {
    pub address: String,
    pub hash: String,
    pub height: u64,
    pub signature: String,
    pub version: u64,
}
