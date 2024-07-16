use mb_btc::blockstream::get_address;
use mb_btc::Error;

#[tokio::test]
async fn test_get_address_ok() {
    let binance_address = String::from("34xp4vRoCGJym3xR7yCVPFHoCNxv4Twseo");
    let res = get_address(&binance_address, 10, None::<&str>).await;
    assert!(res.unwrap().chain.tx_count > 100);
}

#[tokio::test]
async fn test_get_address_wrong_address() {
    let wrong_address = String::from("34xp4vRoCGJym3xR7yCVPFHoCNxv4Twseo1");
    let res = get_address(&wrong_address, 10, None::<&str>).await;
    matches!(res.err().unwrap(), Error::InvalidAddress);
}
