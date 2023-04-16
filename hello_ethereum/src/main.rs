#[tokio::main]
async fn main() -> web3::Result<()> {

    let transport = web3::transports::Http::new("https://mainnet.infura.io/v3/1130d8820d9b4b969d222c78c0a71b29")?;
    let web3 = web3::Web3::new(transport);

    // println!("Calling accounts.");
    // let mut accounts = web3.eth().accounts().await?;
    // println!("Accounts: {:?}", accounts);
    // accounts.push("0x6Fb447Ae94F5180254D436A693907a1f57696900".parse().unwrap());

    // println!("Calling balance.");
    // for account in accounts {
    //     let balance = web3.eth().balance(account, None).await?;
    //     println!("Balance of {:?}: {}", account, balance);
    // }

    get_balance(web3);

    Ok(())
}

async fn get_balance(web3 : web3::Web3<Transport>) {

    let mut accounts = web3.eth().accounts().await?;
    accounts.push("0x6Fb447Ae94F5180254D436A693907a1f57696900".parse().unwrap());
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

}