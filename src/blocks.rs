use crate::{
    models::{Descriptions, Height},
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
}
