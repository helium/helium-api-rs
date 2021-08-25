use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsensusGroupFailureV1 {
    pub delay: u64,
    pub hash: String,
    pub block: u64,
    pub height: u64,
    pub members: Vec<String>,
    pub failed_members: Vec<String>,
    pub signatures: Vec<String>,
}
