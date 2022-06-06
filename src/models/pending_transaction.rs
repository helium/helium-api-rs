use crate::models::transactions::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PendingTransaction {
    pub updated_at: String,
    pub r#type: String,
    pub txn: Transaction,
    pub status: String,
    pub hash: String,
    pub failed_reason: String,
    pub created_at: String,
}
