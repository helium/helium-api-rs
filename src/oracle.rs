use crate::{
    models::{OraclePrediction, OraclePrice},
    *,
};

pub mod prices {
    use super::*;

    /// Fetch all inferred oracle prices
    pub fn all(client: &Client) -> Stream<OraclePrice> {
        client.fetch_stream("/oracle/prices", NO_QUERY)
    }

    /// Get the current valid oracle price
    pub async fn current(client: &Client) -> Result<OraclePrice> {
        client.fetch("/oracle/prices/current", NO_QUERY).await
    }

    /// Get the oracle price that was valid at the given block
    pub async fn at_block(client: &Client, block: u64) -> Result<OraclePrice> {
        client
            .fetch(&format!("/oracle/prices/{}", block), NO_QUERY)
            .await
    }
}

/// Fetches a list of oracle price predictions based on received oracle reports
/// and the current oracle price.
pub async fn predictions(client: &Client) -> Result<Vec<OraclePrediction>> {
    client.fetch("/oracle/predictions", NO_QUERY).await
}

#[cfg(test)]
mod test {
    use super::*;
    use models::Usd;
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::default();
        let prices = oracle::prices::all(&client)
            .take(10)
            .into_vec()
            .await
            .expect("oracle prices");
        assert_eq!(prices.len(), 10);
    }

    #[test]
    async fn current() {
        let client = Client::default();
        let price = oracle::prices::current(&client).await.expect("price");
        assert!(price.block > 0);
    }

    #[test]
    async fn at_block() {
        let client = Client::default();
        let price = oracle::prices::at_block(&client, 763816)
            .await
            .expect("price");
        assert_eq!(price.price, Usd::from(733973329));
    }

    #[test]
    async fn predictions() {
        let client = Client::default();
        let predictions = oracle::predictions(&client).await;
        // predictions may be an empty list
        assert!(predictions.is_ok());
    }
}
