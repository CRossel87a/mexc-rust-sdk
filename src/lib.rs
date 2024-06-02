pub mod utils;
use std::time::{Duration, Instant};
use crate::utils::parse_string_to_f64;
use reqwest::Client;
use serde::Deserialize;

pub const PROD_API_URL: &str = "https://api.mexc.com";

pub struct Mexc {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub client: Client
}

// https://mexcdevelop.github.io/apidocs/spot_v3_en/#header


#[derive(Deserialize, Debug)]
pub struct ServerTime {
    #[serde(rename= "serverTime")]
    pub timestamp: u64
}

#[derive(Deserialize, Debug)]
pub struct ExchangeInfo {
    #[serde(rename= "serverTime")]
    pub timestamp: u64,
    pub symbols: Vec<SymbolInfo>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SymbolInfo {
    #[serde(rename = "baseAsset")]
    base_asset: String,
    
    #[serde(rename = "baseAssetPrecision")]
    base_asset_precision: u32,
    
    #[serde(rename = "baseCommissionPrecision")]
    base_commission_precision: u32,
    
    #[serde(rename = "baseSizePrecision", deserialize_with = "parse_string_to_f64")]
    base_size_precision: f64,
    
    #[serde(rename = "filters")]
    filters: Vec<String>,
    
    #[serde(rename = "fullName")]
    full_name: String,
    
    #[serde(rename = "isMarginTradingAllowed")]
    is_margin_trading_allowed: bool,
    
    #[serde(rename = "isSpotTradingAllowed")]
    is_spot_trading_allowed: bool,
    
    #[serde(rename = "makerCommission", deserialize_with = "parse_string_to_f64")]
    maker_commission: f64,
    
    #[serde(rename = "maxQuoteAmount", deserialize_with = "parse_string_to_f64")]
    max_quote_amount: f64,
    
    #[serde(rename = "maxQuoteAmountMarket", deserialize_with = "parse_string_to_f64")]
    max_quote_amount_market: f64,
    
    #[serde(rename = "orderTypes")]
    order_types: Vec<String>,
    
    #[serde(rename = "permissions")]
    permissions: Vec<String>,
    
    #[serde(rename = "quoteAmountPrecision", deserialize_with = "parse_string_to_f64")]
    quote_amount_precision: f64,
    
    #[serde(rename = "quoteAmountPrecisionMarket", deserialize_with = "parse_string_to_f64")]
    quote_amount_precision_market: f64,
    
    #[serde(rename = "quoteAsset")]
    quote_asset: String,
    
    #[serde(rename = "quoteAssetPrecision")]
    quote_asset_precision: u32,
    
    #[serde(rename = "quoteCommissionPrecision")]
    quote_commission_precision: u32,
    
    #[serde(rename = "quotePrecision")]
    quote_precision: u32,
    
    #[serde(rename = "status")]
    status: String,
    
    #[serde(rename = "symbol")]
    symbol: String,
    
    #[serde(rename = "takerCommission", deserialize_with = "parse_string_to_f64")]
    taker_commission: f64,
}

#[derive(Debug)]
pub struct Level {
    pub px: f64,
    pub sz: f64
}

#[derive(Deserialize, Debug)]
pub struct Orderbook {
    pub timestamp: u64,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>
}

impl Mexc {

    pub fn new(proxy_url: Option<String>) -> anyhow::Result<Self> {

        let client = match proxy_url {
            Some(url) => {
                let proxy = reqwest::Proxy::all(url)?;
                reqwest::Client::builder().proxy(proxy).build()?
            },
            None => reqwest::Client::new()
        };


        Ok(Self {
            api_key: None,
            api_secret: None,
            client
        })
    }

    pub async fn get_server_time(&self) -> anyhow::Result<u64> {
        let url = format!("{PROD_API_URL}/api/v3/time");
        let resp = self.client.get(url).send().await?;

        let st: ServerTime = resp.json().await?;
        Ok(st.timestamp)
    }

    pub async fn ping(&self) -> anyhow::Result<Duration> {
        let url = format!("{PROD_API_URL}/api/v3/ping");

        let t0 = Instant::now();
        let _ = self.client.get(url).send().await?;
        let t1 = Instant::now();

        Ok(t1 - t0)
    }

    pub async fn symbol_info(&self, symbol: &str) -> anyhow::Result<ExchangeInfo> {
        let url = format!("{PROD_API_URL}/api/v3/exchangeInfo?symbol={symbol}");
        let resp = self.client.get(url).send().await?;

        let exchange_info: ExchangeInfo = resp.json().await?;
        Ok(exchange_info)
    }

    pub async fn exchange_info(&self) -> anyhow::Result<ExchangeInfo> {
        let url = format!("{PROD_API_URL}/api/v3/exchangeInfo");
        let resp = self.client.get(url).send().await?;

        let exchange_info: ExchangeInfo = resp.json().await?;
        Ok(exchange_info)
    }

    pub async fn get_spot_orderbook(&self, symbol: &str, depth: Option<u32>) -> anyhow::Result<Orderbook> {

        // limit: default 100; max 5000

        let url = if let Some(limit) = depth {
            format!("{PROD_API_URL}/api/v3/depth?symbol={symbol}&limit={limit}")
        } else {
            format!("{PROD_API_URL}/api/v3/depth?symbol={symbol}")
        };
        let resp = self.client.get(url).send().await?;

        let orderbook: Orderbook = resp.json().await?;
        Ok(orderbook)
    }

}



#[cfg(test)]
mod tests {
    use crate::Mexc;


    #[tokio::test]
    pub async fn test_get_server_time() {

        let client = Mexc::new(None).unwrap();

        let time = client.get_server_time().await.unwrap();
        dbg!(time);
    }

    #[tokio::test]
    pub async fn test_ping() {

        let client = Mexc::new(None).unwrap();

        let dur = client.ping().await.unwrap();
        dbg!(dur);
    }

    
    #[tokio::test]
    pub async fn test_symbol_info() {
        let client = Mexc::new(None).unwrap();
        let info = client.symbol_info("PLSUSDT").await.unwrap();
        dbg!(info);
    }

    #[tokio::test]
    pub async fn test_exchange_info() {
        let client = Mexc::new(None).unwrap();
        let info = client.exchange_info().await.unwrap();
        dbg!(info);
    }


    #[tokio::test]
    pub async fn test_get_spot_orderbook() {
        let client = Mexc::new(None).unwrap();
        let info = client.get_spot_orderbook("PLSUSDT", Some(5)).await.unwrap();
        dbg!(info);
    }

}
