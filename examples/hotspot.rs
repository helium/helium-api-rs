use helium_api::{accounts, hotspots, Client, IntoVec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    let hotspot = hotspots::get(
        &client,
        "11VKaN7fEvDm6NaGhcZtNSU1KAQQmTSwuuJsYYEqzh8mSWkoEUd",
    )
    .await?;
    println!("Hotspot: {:?}", hotspot);

    let account = "13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH";
    let hotspots = accounts::hotspots(&client, account).into_vec().await?;

    println!("Account {} Hotspots: {}", account, hotspots.len());

    Ok(())
}
