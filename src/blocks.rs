use crate::{
    models::{transactions::Transaction, BlockData, BlockStats, Descriptions, Height},
    *,
};

/// Get the current height of the blockchain
pub async fn height(client: &Client) -> Result<u64> {
    let height: Height = client.fetch("/blocks/height", NO_QUERY).await?;
    Ok(height.height)
}

pub async fn stats(client: &Client) -> Result<BlockStats> {
    let stats = client.fetch("/blocks/stats", NO_QUERY).await?;
    Ok(stats)
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

pub async fn block_at_height(client: &Client, height: u64) -> Result<BlockData> {
    let block: BlockData = client
        .fetch(&format!("/blocks/{}", height), NO_QUERY)
        .await?;
    Ok(block)
}

pub async fn block_at_hash(client: &Client, hash: &str) -> Result<BlockData> {
    let block: BlockData = client
        .fetch(&format!("/blocks/hash/{}", hash), NO_QUERY)
        .await?;
    Ok(block)
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
    async fn stats() {
        let client = get_test_client();
        let stats = blocks::stats(&client).await.expect("stats");
        assert!(stats.last_hour.avg > 0.0);
        assert!(stats.last_hour.stddev > 0.0);
        assert!(stats.last_day.avg > 0.0);
        assert!(stats.last_day.stddev > 0.0);
        assert!(stats.last_week.avg > 0.0);
        assert!(stats.last_week.stddev > 0.0);
        assert!(stats.last_month.avg > 0.0);
        assert!(stats.last_month.stddev > 0.0);
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
    async fn block_at_height() {
        let client = get_test_client();
        let block = blocks::block_at_height(&client, 1379987)
            .await
            .expect("block_at_height");
        assert!(block.height == 1379987);
        assert!(block.hash == "6amxWy5inERGrpr3lIG9E-2ZkhOJX60bQafY5NctNv8");
        assert!(block.prev_hash == "3E6pPSnAVNlJMKz-CWtnBpwilLaM3TypxSYAgtaRweo");
        assert!(block.transaction_count == 194);
        assert!(block.time == 1654199928);
        assert!(block.snapshot_hash == "");
    }

    #[test]
    async fn block_at_hash() {
        let client = get_test_client();
        let block = blocks::block_at_hash(&client, "6amxWy5inERGrpr3lIG9E-2ZkhOJX60bQafY5NctNv8")
            .await
            .expect("block_at_hash");
        assert!(block.height == 1379987);
        assert!(block.hash == "6amxWy5inERGrpr3lIG9E-2ZkhOJX60bQafY5NctNv8");
        assert!(block.prev_hash == "3E6pPSnAVNlJMKz-CWtnBpwilLaM3TypxSYAgtaRweo");
        assert!(block.transaction_count == 194);
        assert!(block.time == 1654199928);
        assert!(block.snapshot_hash == "");
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
