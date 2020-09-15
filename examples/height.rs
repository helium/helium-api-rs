use helium_api::Client;

fn main() {
    let client = Client::default();
    match client.get_height() {
        Ok(height) => {
            println!("Block Height: {}", height);
            match client.get_block_transactions(height) {
                Ok((transactions, _cursor)) => {
                    println!("Transactions:");
                    for txn in transactions {
                        println!("{:?}", txn);
                    }
                }
                Err(e) => {
                    println!("Error fetching transactions for block {}: {:?}", height, e);
                }
            }
        }
        Err(e) => {
            println!("Error fetching block height: {:?}", e);
        }
    };
}
