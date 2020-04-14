use helium_api::Client;

fn main() {
    let client = Client::default();
    match client.get_height() {
        Ok(height) => println!("Block Height: {}", height),
        Err(e) => println!("Error fetching block height: {:?}", e)
    }
}