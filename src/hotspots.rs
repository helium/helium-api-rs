use crate::{models::Hotspot, models::QueryTimeRange, models::Reward, *};

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

#[cfg(test)]
mod test {
    use super::*;
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
}
