use crate::*;

/// Get a stream of all ouis
pub fn all(client: &Client) -> Stream<Oui> {
    client.fetch_stream("/ouis", NO_QUERY)
}

/// Get a specific oui
pub async fn get(client: &Client, oui: u64) -> Result<Oui> {
    client.fetch(&format!("/ouis/{}", oui), NO_QUERY).await
}

/// Get the last assigned oui
pub async fn last(client: &Client) -> Result<Oui> {
    client.fetch("/ouis/last", NO_QUERY).await
}

/// Get statistics for ouis
pub async fn stats(client: &Client) -> Result<OuiStats> {
    client.fetch("/ouis/stats", NO_QUERY).await
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::default();
        let ouis = ouis::all(&client).into_vec().await.expect("ouis");
        assert!(ouis.len() > 0);
    }

    #[test]
    async fn get() {
        let client = Client::default();
        let oui = ouis::get(&client, 1).await.expect("oui");
        assert_eq!(
            oui.owner,
            "13tyMLKRFYURNBQqLSqNJg9k41maP1A7Bh8QYxR13oWv7EnFooc"
        );
    }

    #[test]
    async fn last() {
        let client = Client::default();
        let oui = ouis::last(&client).await.expect("oui");
        assert!(oui.oui >= 1,);
    }

    #[test]
    async fn stats() {
        let client = Client::default();
        let stats = ouis::stats(&client).await.expect("stats");
        assert!(stats.count >= 1,);
    }
}
