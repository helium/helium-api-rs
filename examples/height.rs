use helium_api::{blocks, Client, DEFAULT_BASE_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");

    let height = blocks::height(&client).await?;
    println!("Block Height: {}", height);
    Ok(())
}
