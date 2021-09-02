use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PendingTxnStatus {
    pub hash: String,
}
