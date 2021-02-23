use helium_api::{error::Result, Client};

fn main() -> Result<()> {
    let client = Client::default();

    let oracle_price = client.get_oracle_price_current()?;
    println!("Current {:?}", oracle_price);

    let predicted_prices = client.get_oracle_price_predicted()?;
    for predicted_price in predicted_prices {
        println!("{:?}", predicted_price);
    }
    Ok(())
}
