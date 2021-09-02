use super::Reward;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RewardsV2 {
    pub hash: String,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub rewards: Vec<Reward>,
}
