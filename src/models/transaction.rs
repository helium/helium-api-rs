use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct PendingTxnStatus {
    pub hash: String,
}
