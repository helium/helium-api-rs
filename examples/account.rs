use helium_api::Client;

fn main() {
    let client = Client::default();
    let address = "13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH";

    match client.get_account(address) {
        Ok(account) => {
            println!("{:?}", account);
            match client.get_account_transactions(address) {
                Ok((transactions, _cursor)) => {
                    if let Some(transactions) = transactions {
                        println!("Transactions: ");
                        for txn in transactions {
                            println!("{:?}", txn);
                        }
                    }
                }
                Err(e) => {
                    println!("Error querying account {} transactions: {:?}", address, e);
                }
            }
        }
        Err(e) => {
            println!("Error querying account {}: {:?}", address, e);
        }
    }
}