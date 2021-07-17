use crate::{
    models::{Hotspot, QueryTimeRange, Reward, RewardSum},
    *,
};

/// Get all known hotspots
pub fn all(client: &Client) -> Stream<Hotspot> {
    client.fetch_stream("/hotspots", NO_QUERY)
}

/// Get a specific hotspot by its address
pub async fn get(client: &Client, address: &str) -> Result<Hotspot> {
    client
        .fetch(&format!("/hotspots/{}", address), NO_QUERY)
        .await
}

/// Get rewards for a specific hotspot by its address
pub fn rewards(client: &Client, address: &str, query: &QueryTimeRange) -> Stream<Reward> {
    client.fetch_stream(&format!("/hotspots/{}/rewards", address), query)
}

/// Get the sum of rewards for a specific hotspot
pub async fn rewards_sum(
    client: &Client,
    address: &str,
    query: &QueryTimeRange,
) -> Result<RewardSum> {
    client
        .fetch(&format!("/hotspots/{}/rewards/sum", address), query)
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::values::Hnt;
    use rust_decimal::Decimal;
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::default();
        let hotspots =
            hotspots::all(&client)
                .take(10)
                .fold(vec![], |mut acc, hotspot| async move {
                    acc.push(hotspot.unwrap().address);
                    acc
                });
        assert_eq!(hotspots.await.len(), 10);
    }

    #[test]
    async fn get() {
        let client = Client::default();
        let hotspot = hotspots::get(
            &client,
            "112vvSrNAwJRSmR54aqFLEhbr6cy6T4Ufuja4VWVrxvkUAUxL2yG",
        )
        .await
        .expect("hotspot");
        assert_eq!(
            hotspot.address,
            "112vvSrNAwJRSmR54aqFLEhbr6cy6T4Ufuja4VWVrxvkUAUxL2yG"
        );
    }

    #[test]
    async fn rewards() {
        let client = Client::default();
        let params = QueryTimeRange {
            min_time: "2021-06-01".into(),
            max_time: "2021-06-05".into(),
        };
        let rewards = hotspots::rewards(
            &client,
            "112vvSrNAwJRSmR54aqFLEhbr6cy6T4Ufuja4VWVrxvkUAUxL2yG",
            &params,
        )
        .into_vec()
        .await
        .expect("rewards");
        println!("{:?}", rewards);
        assert_eq!(rewards.len(), 21);
    }

    #[test]
    async fn rewards_sum() {
        let client = Client::default();
        let params = QueryTimeRange {
            min_time: "2021-06-01".into(),
            max_time: "2021-06-05".into(),
        };
        let rewards_sum = hotspots::rewards_sum(
            &client,
            "112vvSrNAwJRSmR54aqFLEhbr6cy6T4Ufuja4VWVrxvkUAUxL2yG",
            &params,
        )
        .await
        .expect("rewards_sum");
        println!("{:?}", rewards_sum);
        assert_eq!(rewards_sum.sum, Hnt::new(Decimal::new(143105183, 0)));
    }
}
