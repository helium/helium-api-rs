use crate::{
    models::{transactions::Transaction, Descriptions, Height},
    *,
};

/// Get the current height of the blockchain
pub async fn height(client: &Client) -> Result<u64> {
    let height: Height = client.fetch("/blocks/height", NO_QUERY).await?;
    Ok(height.height)
}

/// Retrieves block descriptions. Blocks descriptions are paged.
/// A cursor field will be in the response when more results are available.
pub async fn descriptions(client: &Client, cursor: Option<&str>) -> Result<Descriptions> {
    let query = cursor.map_or(NO_QUERY.to_vec(), |c| vec![c]);
    client
        .fetch_data("/blocks", &query)
        .await
        .map(|Data { data, cursor }| Descriptions { data, cursor })
}

pub fn transactions_at_height(client: &Client, block: u64) -> Stream<Transaction> {
    client.fetch_stream(format!("/blocks/{}/transactions", block).as_str(), NO_QUERY)
}

pub fn transactions_at_block_hash(client: &Client, hash: &str) -> Stream<Transaction> {
    client.fetch_stream(
        format!("/blocks/hash/{}/transactions", hash).as_str(),
        NO_QUERY,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn height() {
        let client = get_test_client();
        let height = blocks::height(&client).await.expect("height");
        assert!(height > 0);
    }

    #[test]
    async fn descriptions() {
        let client = get_test_client();
        let descriptions = blocks::descriptions(&client, None)
            .await
            .expect("descriptions");
        assert!(descriptions.data.len() > 0);
    }

    #[test]
    async fn transactions_at_height() {
        let client = get_test_client();
        let transactions = blocks::transactions_at_height(&client, 1378232)
            .take(10)
            .into_vec()
            .await
            .expect("transactions");
        assert_eq!(transactions.len(), 10);
    }

    #[test]
    async fn transactions_at_block_hash() {
        let client = get_test_client();
        let transactions = blocks::transactions_at_block_hash(
            &client,
            "BogDArZ5QxbgSd4wLmCS8NRtRzwvCA5fGn1V2TtsYoU",
        )
        .take(10)
        .into_vec()
        .await
        .expect("transactions");
        assert_eq!(transactions.len(), 10);
    }
}
