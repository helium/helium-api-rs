use helium_api::{accounts, hotspots, Client, IntoVec, DEFAULT_BASE_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_with_base_url(DEFAULT_BASE_URL.to_string(), "helium-api-rs/example");

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
