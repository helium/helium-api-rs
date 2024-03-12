use crate::{
    models::{Stats, TokenSupply},
    *,
};

pub async fn stats(client: &Client) -> Result<Stats> {
    let stats: Stats = client.fetch("/stats", NO_QUERY).await?;
    Ok(stats)
}

pub async fn token_supply(client: &Client) -> Result<f64> {
    let token_supply: TokenSupply = client.fetch("/stats/token_supply", NO_QUERY).await?;
    Ok(token_supply.token_supply)
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn stats() {
        let client = get_test_client();
        let stats = stats::stats(&client).await.expect("stats");

        assert!(stats.block_times.last_hour.avg > 0.0);
        assert!(stats.block_times.last_hour.stddev > 0.0);
        assert!(stats.block_times.last_day.avg > 0.0);
        assert!(stats.block_times.last_day.stddev > 0.0);
        assert!(stats.block_times.last_week.avg > 0.0);
        assert!(stats.block_times.last_week.stddev > 0.0);
        assert!(stats.block_times.last_month.avg > 0.0);
        assert!(stats.block_times.last_month.stddev > 0.0);

        assert!(stats.challenge_counts.last_day > 0);

        assert!(stats.election_times.last_hour.avg > 0.0);
        assert!(stats.election_times.last_hour.stddev > 0.0);
        assert!(stats.election_times.last_day.avg > 0.0);
        assert!(stats.election_times.last_day.stddev > 0.0);
        assert!(stats.election_times.last_week.avg > 0.0);
        assert!(stats.election_times.last_week.stddev > 0.0);
        assert!(stats.election_times.last_month.avg > 0.0);
        assert!(stats.election_times.last_month.stddev > 0.0);

        assert!(stats.counts.validators > 0);
        assert!(stats.counts.ouis > 0);
        assert!(stats.counts.hotspots_dataonly > 0);
        assert!(stats.counts.blocks > 0);
        assert!(stats.counts.challenges > 0);
        assert!(stats.counts.cities > 0);
        assert!(stats.counts.consensus_groups > 0);
        assert!(stats.counts.countries > 0);
        assert!(stats.counts.hotspots > 0);
        assert!(stats.counts.transactions > 0);

        assert!(stats.token_supply > 0.0);
    }

    #[test]
    async fn token_supply() {
        let client = get_test_client();
        let token_supply = stats::token_supply(&client).await.expect("stats");
        assert!(token_supply > 0.0);
    }
}
