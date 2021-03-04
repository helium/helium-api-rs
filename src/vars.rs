use crate::*;

pub async fn get(client: &Client) -> Result<serde_json::Map<String, serde_json::Value>> {
    let result: serde_json::Value = client.fetch("/vars", NO_QUERY).await?;
    result
        .as_object()
        .cloned()
        .ok_or_else(|| Error::value(result))
}
