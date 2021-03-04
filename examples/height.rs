use helium_api::{blocks, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    let height = blocks::height(&client).await?;
    println!("Block Height: {}", height);
    Ok(())
}
