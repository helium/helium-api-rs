use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VarsV1 {
    pub hash: String,
    pub vars: serde_json::Value,
    pub unsets: Vec<serde_json::Value>,
    pub cancels: Vec<serde_json::Value>,
    pub nonce: u64,
    pub proof: String,
    pub version_predicate: u64,
    pub master_key: Option<String>,
    pub key_proof: String,
}
