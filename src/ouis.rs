use crate::*;
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Deserialize, Debug)]
/// Represents an OUI on the blockchain
pub struct Oui {
    /// The oui value.
    pub oui: u64,
    /// The base58 public key of the owner of the oui.
    pub owner: String,
    /// The current nonce for the oui
    pub nonce: u64,
    /// The base58 encoded public keys of the routers for this oui
    pub addresses: Vec<String>,
    /// The subnets for this oui
    pub subnets: Vec<Subnet>,
}

#[derive(Clone, Deserialize, Debug)]
/// An OUI owns a list of subnets, which are used to check if packets from a
/// device with a given DevAddr need to be sent to the routers in the OUI
pub struct Subnet {
    base: u32,
    mask: u32,
}

/// Stats for ouis
#[derive(Clone, Deserialize, Debug)]
pub struct Stats {
    pub count: u64,
}

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
pub async fn stats(client: &Client) -> Result<Stats> {
    client.fetch("/ouis/stats", NO_QUERY).await
}

impl fmt::Display for Subnet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.base, self.mask))
    }
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
