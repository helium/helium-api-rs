use helium_api::{oracle, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let price = oracle::prices::current(&client).await?;
    println!("Current: {:?}", price);
    Ok(())
}
