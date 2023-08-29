use helium_api::{models::QueryTimeRange, validators, Client, IntoVec, DEFAULT_BASE_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");

    let stats = validators::stats(&client).await?;
    println!("Stats {:?}", stats);

    // Get all Validators
    let validators = validators::all(&client).into_vec().await?;
    println!("Fetched {} validators.", validators.len());

    // Get Rewards
    if let Some(v) = validators.last() {
        let params = QueryTimeRange {
            min_time: Some("-30 day".into()),
            max_time: Some("-1 hour".into()),
        };
        println!("Validator: {:?}", v);
        let rewards = validators::rewards(&client, &v.address, &params)
            .into_vec()
            .await?;
        println!("Last 10 rewards:");
        rewards
            .iter()
            .take(10)
            .for_each(|r| println!("Block: {}: {} HNT", r.block, r.amount.to_string()));
    };

    Ok(())
}
