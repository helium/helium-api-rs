use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsensusGroupV1 {
    pub delay: u64,
    pub hash: String,
    pub height: u64,
    pub members: Vec<String>,
    pub proof: String,
}
