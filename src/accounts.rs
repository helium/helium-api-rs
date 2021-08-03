use crate::{
    models::{Account, Hotspot, Oui, Transaction, Validator},
    *,
};

/// Get all known accounts
pub fn all(client: &Client) -> Stream<Account> {
    client.fetch_stream("/accounts", NO_QUERY)
}

/// Get a specific account by its address
pub async fn get(client: &Client, address: &str) -> Result<Account> {
    client
        .fetch(&format!("/accounts/{}", address), NO_QUERY)
        .await
}

/// Get all hotspots owned by a given account
pub fn hotspots(client: &Client, address: &str) -> Stream<Hotspot> {
    client.fetch_stream(&format!("/accounts/{}/hotspots", address), NO_QUERY)
}

/// Get all OUIs owned by a given account
pub fn ouis(client: &Client, address: &str) -> Stream<Oui> {
    client.fetch_stream(&format!("/accounts/{}/ouis", address), NO_QUERY)
}

/// Get all validators owned by a given account
pub fn validators(client: &Client, address: &str) -> Stream<Validator> {
    client.fetch_stream(&format!("/accounts/{}/validators", address), NO_QUERY)
}

/// Get a list of of up to a limit (maximum 1000) accounts sorted by their balance in
/// descending order
pub async fn richest(client: &Client, limit: Option<u32>) -> Result<Vec<Account>> {
    client
        .fetch(
            &format!("/accounts/rich?limit={}", limit.unwrap_or(1000)),
            NO_QUERY,
        )
        .await
}

/// Fetches transactions that indicate activity for an account. This includes any
/// transaction that involves the account, usually as a payer, payee or owner.
pub fn activity(client: &Client, address: &str) -> Stream<Transaction> {
    client.fetch_stream(&format!("/accounts/{}/activity", address), NO_QUERY)
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::default();
        let accounts =
            accounts::all(&client)
                .take(10)
                .fold(vec![], |mut acc, account| async move {
                    acc.push(account.unwrap().address);
                    acc
                });
        assert_eq!(accounts.await.len(), 10);
    }

    #[test]
    async fn get() {
        let client = Client::default();
        let account = accounts::get(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
        )
        .await
        .expect("account");
        assert_eq!(
            account.address,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R"
        );
    }

    #[test]
    async fn ouis() {
        let client = Client::default();
        let ouis = accounts::ouis(
            &client,
            "13tyMLKRFYURNBQqLSqNJg9k41maP1A7Bh8QYxR13oWv7EnFooc",
        )
        .into_vec()
        .await
        .expect("oui list");
        assert_eq!(ouis.len(), 1);
    }

    #[test]
    async fn hotspots() {
        let client = Client::default();
        let hotspots = accounts::hotspots(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
        )
        .into_vec()
        .await
        .expect("hotspot list");
        assert!(hotspots.len() > 0);
    }

    #[test]
    async fn richest() {
        let client = Client::default();
        let richest = accounts::richest(&client, Some(10))
            .await
            .expect("richet list");
        assert_eq!(richest.len(), 10);
    }
}
