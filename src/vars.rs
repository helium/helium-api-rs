use crate::*;

pub async fn get(client: &Client) -> Result<serde_json::Map<String, serde_json::Value>> {
    let result: serde_json::Value = client.fetch("/vars", NO_QUERY).await?;
    result
        .as_object()
        .cloned()
        .ok_or_else(|| Error::value(result))
}

pub async fn get_named(
    client: &Client,
    keys: &[&str],
) -> Result<serde_json::Map<String, serde_json::Value>> {
    let param = keys.join(",");
    let result: serde_json::Value = client.fetch("/vars", &[("keys", param)]).await?;
    result
        .as_object()
        .cloned()
        .ok_or_else(|| Error::value(result))
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn named() {
        let client = Client::default();
        let vars = vars::get_named(&client, &["txn_fees"])
            .await
            .expect("named vars");
        assert_eq!(
            true,
            vars.get("txn_fees")
                .expect("txn_fees")
                .as_bool()
                .expect("bool")
        );
    }
}
