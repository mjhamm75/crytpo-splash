use serde::{Deserialize, Serialize};
use log::debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    // pub image: URL,
    pub market_cap: f64,
    pub market_cap_rank: Option<f32>,
    pub fully_diluted_valuation: Option<f64>,
    pub total_volume: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub price_change_24h: f32,
    pub price_change_percentage_24h: f32,
    pub market_cap_change_24h: f64,
    pub market_cap_change_percentage_24h: f32,
    pub circulating_supply: f64,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: f32,
    pub atl: f32,
    pub ath_change_percentage: f32,
    // pub ath_date: Date,
    pub atl_change_percentage: f32,
    // pub alt_date: Date,
    // pub roi: Option<f32>,
    // pub last_updated: Date
}


#[tokio::main]
pub async fn get_coins(coins: Vec<String>) -> Result<Vec<Coin>, Box<dyn std::error::Error>> {
    debug!("{:?}", coins);
    debug!("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&symbols=btc,eth,{}", coins.join(","));
    let resp = reqwest::get(
        // format!("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids={}", coins.join(","))
        format!("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&symbols=btc,eth,{}", coins.join(","))
    )
    .await?
    .json::<Vec<Coin>>()
    .await?;

    debug!("{:?}", resp);

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loads_wasm_file() {
        let result = Module::from_file("./tests/test.wasm");
        assert!(result.is_ok());
    }
}
