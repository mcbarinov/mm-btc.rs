use mb_btc::blockstream::get_address_with_attempts;

#[tokio::main]
async fn main() {
    let binance_address = String::from("34xp4vRoCGJym3xR7yCVPFHoCNxv4Twseo");
    let res = get_address_with_attempts(binance_address, 10, vec![], 10).await.unwrap();
    println!("balance: {}", res.confirmed_balance() as f64 / 100_000_000.0);
}
