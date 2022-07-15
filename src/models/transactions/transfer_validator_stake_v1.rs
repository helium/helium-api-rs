use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TransferValidatorStakeV1 {
    pub fee: u64,
    pub hash: String,
    pub new_address: String,
    pub new_owner: String,
    pub new_owner_signature: Option<String>,
    pub old_address: String,
    pub old_owner: String,
    pub old_owner_signature: String,
    pub payment_amount: Hnt,
    pub stake_amount: Hnt,
}
