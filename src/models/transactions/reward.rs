use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reward {
    pub account: Option<String>,
    pub amount: Hnt,
    pub gateway: Option<String>,
    pub r#type: String,
}
