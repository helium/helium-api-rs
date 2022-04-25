use crate::{models::Height, *};

/// Get the current height of the blockchain
pub async fn height(client: &Client) -> Result<u64> {
    let height: Height = client.fetch("/blocks/height", NO_QUERY).await?;
    Ok(height.height)
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn heigh() {
        let client = get_test_client();
        let height = blocks::height(&client).await.expect("height");
        assert!(height > 0);
    }
}
