use crate::{models::transactions::TransactionRecord, *};

/// Get a specific account by its address
pub async fn get(client: &Client, hash: &str) -> Result<TransactionRecord> {
    client
        .fetch(&format!("/transactions/{}", hash), NO_QUERY)
        .await
}
