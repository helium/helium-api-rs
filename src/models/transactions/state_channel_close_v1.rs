use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StateChannelCloseV1 {
    pub hash: String,
    pub state_channel: StateChannel,
    pub conflicts_with: Option<StateChannel>,
    pub closer: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StateChannel {
    pub summaries: Vec<StateChannelSummary>,
    pub state: String,
    pub root_hash: String,
    pub owner: String,
    pub nonce: u64,
    pub id: String,
    pub expire_at_block: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StateChannelSummary {
    pub num_packets: u64,
    pub num_dcs: u64,
    pub client: String,
}
