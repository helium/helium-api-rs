use helium_api::{accounts, Client, IntoVec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    let account = accounts::get(
        &client,
        "13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH",
    )
    .await?;
    println!("Account: {:?}", account);

    let transactions = accounts::activity(
        &client,
        "13vSgJU5rArGv7SryX9h2n4Rz73LM1Achv1J6eFKgjejoKauPr2",
    )
    .into_vec()
    .await?;

    for txn in transactions {
        println!("{:?}", txn);
    }

    Ok(())
}
