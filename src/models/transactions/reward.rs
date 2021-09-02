use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reward {
    pub account: Option<String>,
    pub amount: u64,
    pub gateway: Option<String>,
    pub r#type: String,
}
