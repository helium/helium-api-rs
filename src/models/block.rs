use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Height {
    /// The current block height of the chain.
    pub height: u64,
}

#[derive(Debug, Deserialize)]
pub struct Descriptions {
    pub data: Vec<BlockData>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BlockData {
    pub transaction_count: u64,
    pub time: u64,
    pub snapshot_hash: String,
    pub prev_hash: String,
    pub height: u64,
    pub hash: String,
}
