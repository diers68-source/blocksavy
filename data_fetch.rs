use serde_json::Value;
use reqwest::Client;

pub async fn fetch_bitcoin_prices() -> Value {
    let url = "https://api.coingecko.com/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days=7";
    let client = Client::new();

    let res = client.get(url)
        .send()
        .await
        .expect("Failed to fetch")
        .json::<Value>()
        .await
        .expect("Failed to parse");

    res
}
