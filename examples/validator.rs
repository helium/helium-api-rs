use helium_api::{validators, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Switch back to mainnet when validators go live
    let client = Client::new_with_base_url("https://testnet-api.helium.wtf/v1".to_string());

    let stats = validators::stats(&client).await?;
    println!("Stats {:?}", stats);
    Ok(())
}
