use crate::{Mexc, PROD_API_URL, utils::parse_string_to_f64};
use serde::Deserialize;
use serde::de::{self, Visitor, SeqAccess};
use std::fmt;
use serde::Deserializer;



#[derive(Deserialize, Debug)]
pub struct Orderbook {
    pub timestamp: u128,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>
}

#[derive(Deserialize, Debug)]
pub struct ExchangeInfo {
    #[serde(rename= "serverTime")]
    pub timestamp: u128,
    pub symbols: Vec<SymbolInfo>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SymbolInfo {
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: u32,
    
    #[serde(rename = "baseCommissionPrecision")]
    pub base_commission_precision: u32,
    
    #[serde(rename = "baseSizePrecision", deserialize_with = "parse_string_to_f64")]
    pub base_size_precision: f64,
    
    #[serde(rename = "filters")]
    pub filters: Vec<String>,
    
    #[serde(rename = "fullName")]
    pub full_name: String,
    
    #[serde(rename = "isMarginTradingAllowed")]
    pub is_margin_trading_allowed: bool,
    
    #[serde(rename = "isSpotTradingAllowed")]
    pub is_spot_trading_allowed: bool,
    
    #[serde(rename = "makerCommission", deserialize_with = "parse_string_to_f64")]
    pub maker_commission: f64,
    
    #[serde(rename = "maxQuoteAmount", deserialize_with = "parse_string_to_f64")]
    pub max_quote_amount: f64,
    
    #[serde(rename = "maxQuoteAmountMarket", deserialize_with = "parse_string_to_f64")]
    pub max_quote_amount_market: f64,
    
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,
    
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    
    #[serde(rename = "quoteAmountPrecision", deserialize_with = "parse_string_to_f64")]
    pub quote_amount_precision: f64,
    
    #[serde(rename = "quoteAmountPrecisionMarket", deserialize_with = "parse_string_to_f64")]
    pub quote_amount_precision_market: f64,
    
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    
    #[serde(rename = "quoteAssetPrecision")]
    pub quote_asset_precision: u32,
    
    #[serde(rename = "quoteCommissionPrecision")]
    pub quote_commission_precision: u32,
    
    #[serde(rename = "quotePrecision")]
    pub quote_precision: u32,
    
    #[serde(rename = "status")]
    pub status: String,
    
    #[serde(rename = "symbol")]
    pub symbol: String,
    
    #[serde(rename = "takerCommission", deserialize_with = "parse_string_to_f64")]
    pub taker_commission: f64,
}

#[derive(Debug)]
pub struct Level {
    pub px: f64,
    pub sz: f64
}

impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LevelVisitor;

        impl<'de> Visitor<'de> for LevelVisitor {
            type Value = Level;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a two-element array [px, sz]")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Level, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let px: String = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let sz: String = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let px: f64 = px.parse().map_err(de::Error::custom)?;
                let sz: f64 = sz.parse().map_err(de::Error::custom)?;

                Ok(Level { px, sz })
            }
        }

        deserializer.deserialize_seq(LevelVisitor)
    }
}

impl Mexc {

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