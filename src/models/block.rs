use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Height {
    /// The current block height of the chain.
    pub height: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlockStats {
    pub last_day: BlockStatsMeasures,
    pub last_hour: BlockStatsMeasures,
    pub last_month: BlockStatsMeasures,
    pub last_week: BlockStatsMeasures,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlockStatsMeasures {
    pub avg: f64,
    pub stddev: f64,
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
