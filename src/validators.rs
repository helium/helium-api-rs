use crate::{
    models::{QueryTimeRange, Reward, Validator, ValidatorStats},
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
pub async fn rewards(client: &Client, address: &str, query: &QueryTimeRange) -> Stream<Reward> {
    client.fetch_stream(&format!("/validators/{}/rewards", address), query)
}
