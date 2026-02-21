use serde::Deserialize;
use std::collections::HashMap;
use reqwest;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct CoinGeckoPrice {
    pub usd: f64,
    #[serde(rename = "usd_24h_change")]
    pub change_24h: f64,
    #[serde(rename = "usd_24h_vol")]
    pub volume_24h: f64,
    #[serde(rename = "usd_market_cap")]
    pub market_cap: f64,
}

pub type CoinGeckoResponse = HashMap<String, CoinGeckoPrice>;

pub fn symbol_to_coingecko_id(symbol: &str) -> Option<&'static str> {
    match symbol.to_uppercase().as_str() {
        "BTC" => Some("bitcoin"),
        "ETH" => Some("ethereum"),
        "SOL" => Some("solana"),
        "USDT" => Some("tether"),
        "USDC" => Some("usd-coin"),
        "BNB" => Some("binancecoin"),
        "XRP" => Some("ripple"),
        "ADA" => Some("cardano"),
        "DOGE" => Some("dogecoin"),
        _ => None,
    }
}

// pub fn coingecko_id_to_symbol(id: &str) -> Option<&'static str> {
//     match id {
//         "bitcoin" => Some("BTC"),
//         "ethereum" => Some("ETH"),
//         "solana" => Some("SOL"),
//         "tether" => Some("USDT"),
//         "usd-coin" => Some("USDC"),
//         "binancecoin" => Some("BNB"),
//         "ripple" => Some("XRP"),
//         "cardano" => Some("ADA"),
//         "dogecoin" => Some("DOGE"),
//         _ => None,
//     }
// }

pub async fn fetch_prices(symbols: &[&str]) -> Result<CoinGeckoResponse, reqwest::Error> {
    let ids: Vec<String> = symbols
        .iter()
        .filter_map(|&s| symbol_to_coingecko_id(s).map(|id| id.to_string()))
        .collect();

    if ids.is_empty() {
        return Ok(HashMap::new());
    }

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true&include_24hr_vol=true&include_market_cap=true",
        ids.join(",")
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("crypto-aggregator/0.1")
        .build()?;

    let response = client.get(&url).send().await?;
    
    // Проверяем статус и возвращаем ошибку если нужно
    let response = response.error_for_status()?;

    let prices: CoinGeckoResponse = response.json().await?;

    Ok(prices)
}