use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Geocode {
    /// The long version of city for the last asserted location
    pub long_city: Option<String>,
    /// The long version of country for the last asserted location
    pub long_country: Option<String>,
    /// The long version of state for the last asserted location
    pub long_state: Option<String>,
    /// The long version of street for the last asserted location
    pub long_street: Option<String>,
    /// The short version of city for the last asserted location
    pub short_city: Option<String>,
    /// The short version of country for the last asserted location
    pub short_country: Option<String>,
    /// The short version of state for the last asserted location
    pub short_state: Option<String>,
    /// The short version of street for the last asserted location
    pub short_street: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hotspot {
    /// The address of the hotspots. This is the public key in base58
    /// check-encoding of the hotspot.
    pub address: String,
    /// The hotspot owner wallet address
    pub owner: String,
    /// The "animal" name of the hotspot. The name can be `None` for
    /// some API endpoints.
    pub name: Option<String>,
    /// The block height when the hotspot was added to the blockchain
    pub added_height: Option<u64>,
    /// The last asserted latitude of the hotspot
    pub lat: Option<f64>,
    /// The last asserted longitude of the hotspot
    pub lng: Option<f64>,
    /// The h3 index based on the lat/lon of the hotspot is used for
    /// PoC challenges.
    pub location: Option<String>, // h3
    /// The geocode information for the hotspot location
    pub geocode: Geocode,
}

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
}
