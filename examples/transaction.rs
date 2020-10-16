use helium_api::Client;

fn main() {
    let client = Client::default();
    let burn_transactions = client.get_transaction("tY-U79IwjjqT-wjvdZqJqIp2Ctyqe5ehuJLMrO91RfI");
    println!("{:?}", burn_transactions);

}
