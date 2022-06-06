use crate::{
    models::{
        transactions::{Challenge, Transaction},
        Account, AccountReward, Hotspot, Oui, PendingTransaction, QueryFilter,
        QueryFilterWithTimeRange, QueryLimitWithTimeRange, QueryTimeRange, Role, RoleCount,
        Validator,
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

/// Fetches transactions that indicate an account as a participant. This includes any transaction that involves the account, usually as a payer, payee or owner.
///
/// ## Examples
///
/// ```
///       use crate::*;
///       use helium_api::{Client, DEFAULT_BASE_URL, models::Role, accounts, models::QueryFilterWithTimeRange, IntoVec, Error};
///       
///       async fn get_roles() -> Result<Vec<Role>, Error> {
///         let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");
///         let roles = accounts::roles(
///            &client,
///            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
///            &QueryFilterWithTimeRange {
///                filter_types: Some("rewards_v2".to_string()),
///                min_time: Some("2022-06-05T00:00:00Z".to_string()),
///                max_time: Some("2022-06-06T00:00:00Z".to_string()),
///                limit: Some(10),
///            },
///         )
///         .into_vec()
///         .await;
///         Ok(roles.unwrap())
///       }
/// ```
///
/// ## API Documentation
/// Find more information about the API call under [`Roles for Account`](https://docs.helium.com/api/blockchain/accounts).
pub fn roles(client: &Client, address: &str, query: &QueryFilterWithTimeRange) -> Stream<Role> {
    client.fetch_stream(&format!("/accounts/{}/roles", address), query)
}

/// Count transactions that indicate activity for an account. This includes any transaction that involves the account, usually as a payer, payee or owner.
///
/// The results are a map keyed by the given `filter_types` and the count of transaction of that type.
/// ## Examples
///
/// ```
///       use crate::*;
///       use helium_api::{Client, DEFAULT_BASE_URL, accounts, models::RoleCount, models::QueryFilter};
///       use core::fmt::Error;
///       async fn get_roles_count() -> Result<RoleCount, Error> {
///         let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");
///         let roles_count = accounts::roles_count(
///            &client,
///            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
///            &QueryFilter {
///                filter_types: Some(
///                    "assert_location_v2,rewards_v1,rewards_v2,payment_v2".to_string(),
///                ),
///            },
///         )
///         .await
///         .expect("role count");
///         Ok(roles_count)
///       }
/// ```
///
/// ## API Documentation
/// Find more information about the API call under [`Roles Counts for Account`](https://docs.helium.com/api/blockchain/accounts).
pub async fn roles_count(client: &Client, address: &str, query: &QueryFilter) -> Result<RoleCount> {
    let count = client
        .fetch(&format!("/accounts/{}/roles/count", address), query)
        .await;
    match count {
        Ok(count) => Ok(count),
        Err(err) => Err(err),
    }
}

/// Fetches challenges that hotspots owned by the given account are involved in as a challenger, challengee, or witness.
///
/// ## Examples
///
/// ```
///       use crate::*;
///       use helium_api::{Client, DEFAULT_BASE_URL, accounts, models::transactions::Challenge, models::QueryLimitWithTimeRange, IntoVec, Error};
///       async fn get_challenges() -> Result<Vec<Challenge>, Error> {
///         let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");
///         let challenges = accounts::challenges(
///            &client,
///            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
///            &QueryLimitWithTimeRange {
///               min_time: Some("-7 day".into()),
///               max_time: Some("now".into()),
///               limit: Some(10)
///            },
///         )
///         .into_vec()
///         .await
///         .expect("challenges");
///         Ok(challenges)
///       }
/// ```
///
/// ## API Documentation
/// Find more information about the API call under [`Challenges for Account`](https://docs.helium.com/api/blockchain/accounts).
pub fn challenges(
    client: &Client,
    address: &str,
    query: &QueryLimitWithTimeRange,
) -> Stream<Challenge> {
    client.fetch_stream(&format!("/accounts/{}/challenges", address), query)
}

/// Fetches the outstanding transactions for the given account. See Pending Transactions for details.
///
/// ## Examples
/// ```
///       use crate::*;
///       use helium_api::{Client, DEFAULT_BASE_URL, accounts, models::PendingTransaction, models::QueryLimitWithTimeRange, IntoVec, Error};
///       async fn get_pending_transactions() -> Result<Vec<PendingTransaction>, Error> {
///         let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");
///         let pending_transactions = accounts::pending_transactions(
///            &client,
///            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
///         )
///         .into_vec()
///         .await
///         .expect("pending_transactions");
///         Ok(pending_transactions)
///       }
/// ```
/// ## API Documentation
/// Find more information about the API call under [`Pending Transactions for Account`](https://docs.helium.com/api/blockchain/accounts).
pub fn pending_transactions(client: &Client, address: &str) -> Stream<PendingTransaction> {
    client.fetch_stream(
        &format!("/accounts/{}/pending_transactions", address),
        NO_QUERY,
    )
}

///Returns reward entries by block and gateway for a given account in a timeframe. Timestamps are given in *ISO 8601* format. The block that contains the `max_time` timestamp is excluded from the result.
///
///The result will be a list of rewards between `min_time` and `max_time` both given in UTC. Both default to "now" which means that at least one of the two parameters is required.
///
///The result includes a `type` field which is the type of activity that generated the reward.
///
/// ### Note:
/// For older reward results, if the `type` is None the amount is a total for that account or hotspot in the given block.
///
///## Examples
///```
///      use crate::*;
///      use helium_api::{Client, DEFAULT_BASE_URL, accounts, models::AccountReward, models::QueryTimeRange, IntoVec, Error};
///      async fn get_rewards() -> Result<Vec<AccountReward>, Error> {
///        let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");
///        let account_rewards = accounts::rewards(
///           &client,
///           "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
///           &QueryTimeRange {
///                min_time: Some("-7 day".into()),
///                max_time: Some("-1 day".into()),
///            }
///        )
///        .into_vec()
///        .await
///        .expect("account rewards");
///        Ok(account_rewards)
///      }
///```
///## API Documentation
///Find more information about the API call under [`Rewards for an Account`](https://docs.helium.com/api/blockchain/accounts).
pub fn rewards(client: &Client, address: &str, query: &QueryTimeRange) -> Stream<AccountReward> {
    client.fetch_stream(&format!("/accounts/{}/rewards", address), query)
}

///Returns rewards for a given account for a given block.
///
/// The result includes a `type` field which is the type of activity that generated the reward.
///
/// #### Note:
/// For older reward results, if the `type` is None the amount is a total for that account or hotspot in the given block.
///## Examples
///```
///      use crate::*;
///      use helium_api::{Client, DEFAULT_BASE_URL, accounts, models::AccountReward, models::QueryTimeRange, IntoVec, Error};
///      async fn get_rewards_at_block() -> Result<Vec<AccountReward>, Error> {
///        let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");
///        let account_rewards = accounts::rewards_at_block(
///           &client,
///           "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
///           1342108
///        )
///        .into_vec()
///        .await
///        .expect("account rewards_at_block");
///        Ok(account_rewards)
///      }
///```
///## API Documentation
///Find more information about the API call under [`Rewards in a Rewards Block for an Account`](https://docs.helium.com/api/blockchain/accounts).
pub fn rewards_at_block(client: &Client, address: &str, block: u64) -> Stream<AccountReward> {
    client.fetch_stream(
        &format!("/accounts/{}/rewards/{}", address, block),
        NO_QUERY,
    )
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

    #[test]
    async fn roles_count() {
        let client = get_test_client();
        let roles_count = accounts::roles_count(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
            &QueryFilter { filter_types: None },
        )
        .await
        .expect("role count");

        assert!(roles_count.vars_v1 >= Some(0));
        assert!(roles_count.gen_validator_v1 >= Some(0));
        assert!(roles_count.price_oracle_v1 >= Some(0));
        assert!(roles_count.security_exchange_v1 >= Some(0));
        assert!(roles_count.gen_gateway_v1 >= Some(0));
        assert!(roles_count.consensus_group_v1 >= Some(0));
        assert!(roles_count.token_burn_exchange_rate_v1 >= Some(0));
        assert!(roles_count.transfer_hotspot_v2 >= Some(0));
        assert!(roles_count.poc_receipts_v1 >= Some(0));
        assert!(roles_count.validator_heartbeat_v1 >= Some(0));
        assert!(roles_count.create_htlc_v1 >= Some(0));
        assert!(roles_count.transfer_validator_stake_v1 >= Some(0));
        assert!(roles_count.stake_validator_v1 >= Some(0));
        assert!(roles_count.routing_v1 >= Some(0));
        assert!(roles_count.poc_request_v1 >= Some(0));
        assert!(roles_count.payment_v1 >= Some(0));
        assert!(roles_count.assert_location_v2 > Some(0));
        assert!(roles_count.security_coinbase_v1 >= Some(0));
        assert!(roles_count.assert_location_v1 > Some(0));
        assert!(roles_count.token_burn_v1 >= Some(0));
        assert!(roles_count.rewards_v1 > Some(0));
        assert!(roles_count.unstake_validator_v1 >= Some(0));
        assert!(roles_count.oui_v1 >= Some(0));
        assert!(roles_count.state_channel_open_v1 >= Some(0));
        assert!(roles_count.rewards_v2 > Some(0));
        assert!(roles_count.coinbase_v1 >= Some(0));
        assert!(roles_count.add_gateway_v1 >= Some(0));
        assert!(roles_count.poc_receipts_v2 >= Some(0));
        assert!(roles_count.consensus_group_failure_v1 >= Some(0));
        assert!(roles_count.payment_v2 > Some(0));
        assert!(roles_count.transfer_hotspot_v1 >= Some(0));
        assert!(roles_count.dc_coinbase_v1 >= Some(0));
        assert!(roles_count.state_channel_close_v1 >= Some(0));
        assert!(roles_count.redeem_htlc_v1 >= Some(0));
    }

    #[test]
    async fn roles_count_filter() {
        let client = get_test_client();
        let roles_count = accounts::roles_count(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
            &QueryFilter {
                filter_types: Some(
                    "assert_location_v2,rewards_v1,rewards_v2,payment_v2".to_string(),
                ),
            },
        )
        .await
        .expect("role count");

        assert!(roles_count.vars_v1 == None);
        assert!(roles_count.gen_validator_v1 == None);
        assert!(roles_count.price_oracle_v1 == None);
        assert!(roles_count.security_exchange_v1 == None);
        assert!(roles_count.gen_gateway_v1 == None);
        assert!(roles_count.consensus_group_v1 == None);
        assert!(roles_count.token_burn_exchange_rate_v1 == None);
        assert!(roles_count.transfer_hotspot_v2 == None);
        assert!(roles_count.poc_receipts_v1 == None);
        assert!(roles_count.validator_heartbeat_v1 == None);
        assert!(roles_count.create_htlc_v1 == None);
        assert!(roles_count.transfer_validator_stake_v1 == None);
        assert!(roles_count.stake_validator_v1 == None);
        assert!(roles_count.routing_v1 == None);
        assert!(roles_count.poc_request_v1 == None);
        assert!(roles_count.payment_v1 == None);
        assert!(roles_count.security_coinbase_v1 == None);
        assert!(roles_count.assert_location_v1 == None);
        assert!(roles_count.token_burn_v1 == None);
        assert!(roles_count.unstake_validator_v1 == None);
        assert!(roles_count.oui_v1 == None);
        assert!(roles_count.state_channel_open_v1 == None);
        assert!(roles_count.coinbase_v1 == None);
        assert!(roles_count.add_gateway_v1 == None);
        assert!(roles_count.poc_receipts_v2 == None);
        assert!(roles_count.consensus_group_failure_v1 == None);
        assert!(roles_count.transfer_hotspot_v1 == None);
        assert!(roles_count.dc_coinbase_v1 == None);
        assert!(roles_count.state_channel_close_v1 == None);
        assert!(roles_count.redeem_htlc_v1 == None);

        assert!(roles_count.assert_location_v2 > Some(0));
        assert!(roles_count.payment_v2 > Some(0));
        assert!(roles_count.rewards_v1 > Some(0));
        assert!(roles_count.rewards_v2 > Some(0));
    }

    #[test]
    async fn challenges() {
        let client = get_test_client();
        let challenges = accounts::challenges(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
            &QueryLimitWithTimeRange {
                min_time: Some("-7 day".into()),
                max_time: Some("-1 day".into()),
                limit: Some(10),
            },
        )
        .into_vec()
        .await
        .expect("challenges");

        assert!(challenges.len() == 10);
    }

    #[test]
    async fn pending_transactions() {
        let client = get_test_client();
        let pending_transactions = accounts::pending_transactions(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
        )
        .into_vec()
        .await
        .expect("pending transactions");

        assert!(pending_transactions.len() > 0);
    }

    #[test]
    async fn rewards() {
        let client = get_test_client();
        let rewards = accounts::rewards(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
            &QueryTimeRange {
                min_time: Some("-7 day".into()),
                max_time: Some("-1 day".into()),
            },
        )
        .into_vec()
        .await
        .expect("rewards");

        assert!(rewards.len() > 0);
    }

    #[test]
    async fn rewards_at_block() {
        let client = get_test_client();
        let rewards = accounts::rewards_at_block(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
            1342108,
        )
        .into_vec()
        .await
        .expect("rewards");

        assert!(rewards.len() > 0);
        assert!(rewards[0].block == 1342108);
    }
}
