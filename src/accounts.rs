use crate::{
    models::{
        transactions::Transaction, Account, Hotspot, Oui, QueryFilterWithTimeRange, QueryTimeRange,
        Role, Validator,
    },
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
        .fetch("/accounts/rich", &[("limit", limit.unwrap_or(1000))])
        .await
}

/// Fetches transactions that indicate activity for an account. This includes any
/// transaction that involves the account, usually as a payer, payee or owner.
pub fn activity(client: &Client, address: &str, query: &QueryTimeRange) -> Stream<Transaction> {
    client.fetch_stream(&format!("/accounts/{}/activity", address), query)
}

/// Returns the roles for an account as a Stream
///
/// ## Examples
///
/// ```
///        let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");
///        let roles = accounts::roles(
///            &client,
///            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
///            &QueryFilterWithTimeRange {
///                filter_types: Some("rewards_v2".to_string()),
///                min_time: Some("2022-06-05T00:00:00Z".to_string()),
///                max_time: Some("2022-06-06T00:00:00Z".to_string()),
///                limit: Some(10),
///            },
///        )
///        .into_vec()
///        .await
///        .expect("role list");
/// ```
///
/// ## API Documentation
/// Find more information about the API call under [`Roles for Account`](https://docs.helium.com/api/blockchain/accounts).
pub fn roles(client: &Client, address: &str, query: &QueryFilterWithTimeRange) -> Stream<Role> {
    client.fetch_stream(&format!("/accounts/{}/roles", address), query)
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn all() {
        let client = get_test_client();
        let accounts = accounts::all(&client)
            .take(10)
            .into_vec()
            .await
            .expect("accounts");
        assert_eq!(accounts.len(), 10);
    }

    #[test]
    async fn get() {
        let client = get_test_client();
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
        let client = get_test_client();
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
        let client = get_test_client();
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
        let client = get_test_client();
        let richest = accounts::richest(&client, Some(10))
            .await
            .expect("richest list");
        assert_eq!(richest.len(), 10);
    }

    #[test]
    async fn roles() {
        let client = get_test_client();
        let roles = accounts::roles(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
            &QueryFilterWithTimeRange {
                filter_types: None,
                min_time: Some("2022-06-01T00:00:00Z".to_string()),
                max_time: Some("2022-06-06T00:00:00Z".to_string()),
                limit: None,
            },
        )
        .into_vec()
        .await
        .expect("role list");

        assert!(roles.len() > 0);
    }

    #[test]
    async fn roles_filter() {
        let client = get_test_client();
        let roles = accounts::roles(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
            &QueryFilterWithTimeRange {
                filter_types: Some("rewards_v2".to_string()),
                min_time: Some("2022-06-01T00:00:00Z".to_string()),
                max_time: Some("2022-06-06T00:00:00Z".to_string()),
                limit: Some(10),
            },
        )
        .into_vec()
        .await
        .expect("role list");

        roles.iter().for_each(|role| {
            assert_eq!(role.role_type, "rewards_v2");
        });
        assert!(roles.len() == 10);
    }
}
