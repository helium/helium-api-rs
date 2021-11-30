use helium_api::{oracle, Client, DEFAULT_BASE_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");

    let price = oracle::prices::current(&client).await?;
    println!("Current: {:?}", price);
    Ok(())
}
