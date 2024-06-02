use crate::{Mexc, PROD_API_URL, utils::{parse_string_to_f64, get_timestamp}};
use anyhow::{anyhow, bail};
use reqwest::{StatusCode, Response};
use serde::Deserialize;
use hmac::{Hmac, Mac};
use sha2::Sha256;



#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
pub enum OrderType {
    LIMIT,
    MARKET,
    LIMIT_MAKER,
    IMMEDIATE_OR_CANCEL,
    FILL_OR_KILL
}
impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::LIMIT => write!(f, "LIMIT"),
            OrderType::MARKET => write!(f, "MARKET"),
            OrderType::LIMIT_MAKER => write!(f, "LIMIT_MAKER"),
            OrderType::IMMEDIATE_OR_CANCEL => write!(f, "IMMEDIATE_OR_CANCEL"),
            OrderType::FILL_OR_KILL => write!(f, "FILL_OR_KILL"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum OrderSide {
    BUY,
    SELL
}
impl std::fmt::Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderSide::BUY => write!(f, "BUY"),
            OrderSide::SELL => write!(f, "SELL"),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct OrderReceipe {
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub price: f64,
    #[serde(rename = "origQty", deserialize_with = "parse_string_to_f64")]
    pub orig_qty: f64,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    #[serde(rename = "transactTime")]
    pub transact_time: u128,
}

impl Mexc {

    fn sign_request(&self, order_details: String) -> anyhow::Result<String> {
        let secret_key = self.api_secret.as_ref().ok_or_else(|| anyhow!("Missing secret key"))?;
        let mut signed_key = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())?;
        signed_key.update(order_details.as_bytes());
        let signature = hex::encode(signed_key.finalize().into_bytes());
        let signed_order_details: String = format!("{}&signature={}", order_details, signature);
        Ok(signed_order_details)
    }

    async fn post_signed(&self, url: &str) -> anyhow::Result<Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;

        //let header = format!("X-MEXC-APIKEY: {}", api_key);
        let resp = self.client
        .post(url)
        .header("X-MEXC-APIKEY", api_key)
        .send().await?;
        Ok(resp)
    }

    pub async fn new_order(&self, symbol: &str, side: OrderSide, order_type: OrderType, price: f64, quantity: f64, recv_window: Option<u64>) -> anyhow::Result<OrderReceipe> {
        let recv_window = recv_window.unwrap_or(5000);
        let timestamp = get_timestamp();

        let order_request = format!("symbol={symbol}&side={side}&type={order_type}&quantity={quantity}&price={price}&recvWindow={recv_window}&timestamp={timestamp}");

        let signed_order = self.sign_request(order_request).unwrap();
        //println!("{}",signed_order);

        let url = format!("{PROD_API_URL}/api/v3/order?{signed_order}");
        //println!("{}",url);

        let resp: Response = self.post_signed(&url).await?;

        if resp.status() == StatusCode::OK {
            let receipe: OrderReceipe = resp.json().await?;
            Ok(receipe)
        } else {
            let err = resp.text().await?;
            bail!(err);
        }
    }
}