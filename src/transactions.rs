use crate::{models::transactions::Transaction, *};

/// Get a specific account by its address
pub async fn get(client: &Client, hash: &str) -> Result<Transaction> {
    client
        .fetch(&format!("/transactions/{}", hash), NO_QUERY)
        .await
}
