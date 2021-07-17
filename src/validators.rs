use crate::{
    models::{QueryTimeRange, Reward, RewardSum, Validator, ValidatorStats},
    *,
};

/// Get all known validators
pub fn all(client: &Client) -> Stream<Validator> {
    client.fetch_stream("/validators", NO_QUERY)
}

/// Get a specific validator
pub async fn get(client: &Client, address: &str) -> Result<Validator> {
    client
        .fetch(&format!("/validators/{}", address), NO_QUERY)
        .await
}

/// Get stats for validators
pub async fn stats(client: &Client) -> Result<ValidatorStats> {
    client.fetch("/validators/stats", NO_QUERY).await
}

/// Get rewards for a validator
///
/// Returns rewards for a given validator per reward block the validator is in,
/// for a given timeframe. `QueryTimeRange` contains the timestamps given in
/// 4ISO 8601 format, or in relative time. The block that contains the max_time
/// timestamp is excluded from the result.
pub fn rewards(client: &Client, address: &str, query: &QueryTimeRange) -> Stream<Reward> {
    client.fetch_stream(&format!("/validators/{}/rewards", address), query)
}

/// Get the sum of rewards for a specific validator
pub async fn rewards_sum(
    client: &Client,
    address: &str,
    query: &QueryTimeRange,
) -> Result<RewardSum> {
    client
        .fetch(&format!("/validators/{}/rewards/sum", address), query)
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal::Decimal;
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::default();
        let validators =
            validators::all(&client)
                .take(10)
                .fold(vec![], |mut acc, validator| async move {
                    acc.push(validator.unwrap().address);
                    acc
                });
        assert_eq!(validators.await.len(), 10);
    }

    #[test]
    async fn get() {
        let client = Client::default();
        let validator = validators::get(
            &client,
            "112LY6VZbt9RissyDe9WMSmf6f1P7is1yL6MnwA4nrkWwtmEMiS1",
        )
        .await
        .expect("validator");
        assert_eq!(
            validator.address,
            "112LY6VZbt9RissyDe9WMSmf6f1P7is1yL6MnwA4nrkWwtmEMiS1"
        );
    }

    #[test]
    async fn rewards() {
        // TODO: Switch back to mainnet when validators go live
        let client = Client::new_with_base_url("https://testnet-api.helium.wtf/v1".to_string());
        let params = QueryTimeRange {
            min_time: "2021-06-25".into(),
            max_time: "2021-06-30".into(),
        };
        let rewards = validators::rewards(
            &client,
            "1ZbRKVmsKkJgF6oaPGz8gZEfC6M9fbyHybCp47n7jF69xzLGc9X",
            &params,
        )
        .into_vec()
        .await
        .expect("rewards");
        println!("{:?}", rewards);
        assert_eq!(rewards.len(), 0);
    }

    #[test]
    async fn rewards_sum() {
        // TODO: Switch back to mainnet when validators go live
        let client = Client::new_with_base_url("https://testnet-api.helium.wtf/v1".to_string());
        let params = QueryTimeRange {
            min_time: "2021-07-01".into(),
            max_time: "2021-07-03".into(),
        };
        let rewards_sum = validators::rewards_sum(
            &client,
            "1ZbRKVmsKkJgF6oaPGz8gZEfC6M9fbyHybCp47n7jF69xzLGc9X",
            &params,
        )
        .await
        .expect("rewards_sum");
        println!("{:?}", rewards_sum);
        assert_eq!(rewards_sum.sum, Hnt::new(Decimal::new(61111111116, 0)));
    }
}
