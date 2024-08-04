use crate::{Mexc, PROD_API_URL, utils::{parse_string_to_f64, get_timestamp, serialize_f64_as_string}};
use anyhow::{anyhow, bail};
use reqwest::{StatusCode, Response};
use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub const DEFAULT_RECV_WINDOW: u64 = 5000;



#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
pub enum OrderStatus {
    NEW,
    FILLED,
    PARTIALLY_FILLED,
    CANCELED,
    PARTIALLY_CANCELED
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct OrderReceipt {
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CancelledOrder {
    pub symbol: String,
    //#[serde(rename = "origClientOrderId")]
    //pub orig_client_order_id: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    //#[serde(rename = "clientOrderId")]
    //pub client_order_id: String,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub price: f64,
    #[serde(rename = "origQty", deserialize_with = "parse_string_to_f64")]
    pub orig_qty: f64,
    #[serde(rename = "executedQty", deserialize_with = "parse_string_to_f64")]
    pub exec_qty: f64,
    #[serde(rename = "cummulativeQuoteQty", deserialize_with = "parse_string_to_f64")]
    pub cum_quote_qty: f64,
    //#[serde(rename = "timeInForce")]
    //pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
}


#[derive(Serialize,Deserialize, Debug)]
pub struct Order {
    pub symbol: String,
    #[serde(serialize_with = "serialize_f64_as_string")]
    pub price: f64,
    #[serde(serialize_with = "serialize_f64_as_string")]
    pub quantity: f64,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct OrderQuery {
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: String,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub price: f64,

    #[serde(rename = "origQty", deserialize_with = "parse_string_to_f64")]
    pub orig_qty: f64,

    #[serde(rename = "executedQty", deserialize_with = "parse_string_to_f64")]
    pub exec_qty: f64,

    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,

    #[serde(rename = "time")]
    pub created_time: u128,

    #[serde(rename = "updateTime")]
    pub last_update: Option<u128>,
}

impl Mexc {

    pub fn sign_request(&self, order_details: String) -> anyhow::Result<String> {
        let secret_key = self.api_secret.as_ref().ok_or_else(|| anyhow!("Missing secret key"))?;
        let mut signed_key = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())?;
        signed_key.update(order_details.as_bytes());
        let signature = hex::encode(signed_key.finalize().into_bytes());
        let signed_order_details: String = format!("{}&signature={}", order_details, signature);
        Ok(signed_order_details)
    }

    /*
    pub fn get_blank_signature(&self) -> anyhow::Result<String> {
        let secret_key = self.api_secret.as_ref().ok_or_else(|| anyhow!("Missing secret key"))?;
        let signed_key = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())?;
        let signature = hex::encode(signed_key.finalize().into_bytes());
        let signed_order_details: String = format!("signature={}", signature);
        Ok(signed_order_details)
    }
    */
    
    pub async fn post_signed(&self, url: &str) -> anyhow::Result<Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;

        let resp = self.client
        .post(url)
        .header("X-MEXC-APIKEY", api_key)
        .send().await?;
        Ok(resp)
    }

    pub async fn put_signed(&self, url: &str) -> anyhow::Result<Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;

        let resp = self.client
        .put(url)
        .header("X-MEXC-APIKEY", api_key)
        .send().await?;
        Ok(resp)
    }

    pub async fn delete_signed(&self, url: &str) -> anyhow::Result<Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;
        
        let resp = self.client
        .delete(url)
        .header("X-MEXC-APIKEY", api_key)
        .send().await?;
        Ok(resp)
    }

    pub async fn submit_order(&self, symbol: &str, side: OrderSide, order_type: OrderType, price: f64, quantity: f64, recv_window: Option<u64>) -> anyhow::Result<OrderReceipt> {
        let recv_window = recv_window.unwrap_or(DEFAULT_RECV_WINDOW);
        let timestamp = get_timestamp();

        let order_request = format!("symbol={symbol}&side={side}&type={order_type}&quantity={quantity}&price={price}&recvWindow={recv_window}&timestamp={timestamp}");
        let signed_order = self.sign_request(order_request)?;
        let url = format!("{PROD_API_URL}/api/v3/order?{signed_order}");
        let resp: Response = self.post_signed(&url).await?;

        if resp.status() == StatusCode::OK {
            let receipe: OrderReceipt = resp.json().await?;
            Ok(receipe)
        } else {
            let err = resp.text().await?;
            bail!(err);
        }
    }

    pub async fn batch_orders(&self, orders: Vec<Order>, recv_window: Option<u64>) -> anyhow::Result<Vec<OrderReceipt>> {
        if orders.is_empty() {
            bail!("No orders in vector");
        }

        let recv_window = recv_window.unwrap_or(DEFAULT_RECV_WINDOW);
        let timestamp = get_timestamp();

        let json = serde_json::to_string(&orders)?;

        let encoded_orders = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("batchOrders", &json)
        .finish();

        let order_request = format!("{encoded_orders}&recvWindow={recv_window}&timestamp={timestamp}");

        let signed_order = self.sign_request(order_request)?;
        let url = format!("{PROD_API_URL}/api/v3/batchOrders?{signed_order}");

        let resp: Response = self.post_signed(&url).await?;

        if resp.status() == StatusCode::OK {


            let txt = resp.text().await?;
        
            let res: Result<Vec<OrderReceipt>, _> = serde_json::from_str(&txt);

            match res {
                Ok(receipts) => Ok(receipts),
                Err(err) => {
                    bail!("Err: {err} on {txt}");
                }
            }
        } else {
            let err = resp.text().await?;
            bail!(err);
        }
    }

    pub async fn cancel_all_orders(&self, symbol: &str, recv_window: Option<u64>) -> anyhow::Result<Vec<CancelledOrder>> {
        let recv_window = recv_window.unwrap_or(DEFAULT_RECV_WINDOW);
        let timestamp = get_timestamp();

        let order_request = format!("symbol={symbol}&recvWindow={recv_window}&timestamp={timestamp}");
        let signed_order = self.sign_request(order_request)?;
        let url = format!("{PROD_API_URL}/api/v3/openOrders?{signed_order}");
        let resp: Response = self.delete_signed(&url).await?;

        if resp.status() == StatusCode::OK {
            let cancelled_orders: Vec<CancelledOrder> = resp.json().await?;
            Ok(cancelled_orders)
        } else {
            let err = resp.text().await?;
            bail!(err);
        }
    }

    pub async fn cancel_order(&self, symbol: &str, order_id: &str,recv_window: Option<u64>) -> anyhow::Result<CancelledOrder> {
        let recv_window = recv_window.unwrap_or(DEFAULT_RECV_WINDOW);
        let timestamp = get_timestamp();

        let order_request = format!("symbol={symbol}&orderId={order_id}&recvWindow={recv_window}&timestamp={timestamp}");
        let signed_order = self.sign_request(order_request)?;
        let url = format!("{PROD_API_URL}/api/v3/order?{signed_order}");
        let resp: Response = self.delete_signed(&url).await?;

        if resp.status() == StatusCode::OK {
            let cancelled_order: CancelledOrder = resp.json().await?;
            Ok(cancelled_order)
        } else {
            let err = resp.text().await?;
            bail!(err);
        }
    }

    pub async fn get_open_orders(&self, symbol: &str,recv_window: Option<u64>) -> anyhow::Result<Vec<OrderQuery>> {

        let recv_window = recv_window.unwrap_or(DEFAULT_RECV_WINDOW);
        let timestamp = get_timestamp();

        let order_request = format!("symbol={symbol}&recvWindow={recv_window}&timestamp={timestamp}");
        let signed_order = self.sign_request(order_request)?;
        let url = format!("{PROD_API_URL}/api/v3/openOrders?{signed_order}");
        let resp: Response = self.get_signed(&url).await?;

        if resp.status() == StatusCode::OK {
            let orders: Vec<OrderQuery> = resp.json().await?;
            Ok(orders)
        } else {
            let err = resp.text().await?;
            bail!(err);
        }
    }
}