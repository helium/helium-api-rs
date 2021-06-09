use futures::stream::StreamExt;
use helium_api::{
    validators::{self, QueryTimeRange, Validator},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Switch back to mainnet when validators go live
    let client = Client::new_with_base_url("https://testnet-api.helium.wtf/v1".to_string());

    let stats = validators::stats(&client).await?;
    println!("Stats {:?}", stats);

    // Get all Validators
    let validators = get_all_validators(&client).await;
    println!("Fetched {} validators.", validators.len());

    // Get Rewards
    if let Some(v) = validators.last() {
        let params = QueryTimeRange {
            min_time: "-30 day".into(),
            max_time: "-1 hour".into(),
        };
        let mut stream = validators::rewards(&client, &v.address, &params).await;
        while let Some(r) = stream.next().await {
            match r {
                Ok(r) => println!("{:?}", r),
                Err(e) => println!("Error trying to get helium rewards: {}", e),
            }
        }
    };

    Ok(())
}

async fn get_all_validators(client: &Client) -> Vec<Validator> {
    let mut stream = validators::all(&client);
    let mut validators: Vec<Validator> = Vec::new();

    while let Some(v) = stream.next().await {
        match v {
            Ok(v) => validators.push(v),
            Err(e) => println!("Error trying to get helium validators: {}", e),
        }
    }

    validators
}
