use helium_api::{ouis, Client, DEFAULT_BASE_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");

    let stats = ouis::stats(&client).await?;
    println!("Stats {:?}", stats);

    let oui = ouis::get(&client, 1).await?;
    println!("{:?}", oui);

    Ok(())
}
