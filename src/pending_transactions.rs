use crate::{models::transactions::PendingTxnStatus, *};
use serde_json::json;

/// Convert a given transaction to json, ready to be submitted
/// Submit a transaction to the blockchain
pub async fn submit<T>(client: &Client, txn: T) -> Result<PendingTxnStatus>
where
    T: AsRef<[u8]>,
{
    let json = json!({ "txn": base64::encode(&txn) });
    client.post("/pending_transactions", &json).await
}

/// Get the status for a specific pending transaction hash
pub async fn get(client: &Client, hash: &str) -> Result<PendingTxnStatus> {
    client
        .fetch(&format!("/pending_transactions/{}", hash), NO_QUERY)
        .await
}
