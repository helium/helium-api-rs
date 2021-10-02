use crate::{models::transactions::ProcessedTransaction, *};

/// Get a specific account by its address
pub async fn get(client: &Client, hash: &str) -> Result<ProcessedTransaction> {
    client
        .fetch(&format!("/transactions/{}", hash), NO_QUERY)
        .await
}
