use helium_api::{ouis, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let stats = ouis::stats(&client).await?;
    println!("Stats {:?}", stats);

    let oui = ouis::get(&client, 1).await?;
    println!("{:?}", oui);

    Ok(())
}
