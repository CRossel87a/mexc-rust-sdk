pub mod structures;

use serde_json::json;
use anyhow::Context;
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use reqwest::Response;
use sha2::Sha256;
use reqwest::Client;
use std::time::Duration;
use std::time::Instant;
use anyhow::{anyhow, bail};
use reqwest::header::{HeaderMap, HeaderValue};
use crate::utils::get_timestamp;


use structures::*;

pub const FUTURES_API_URL: &str = "https://contract.mexc.com";

pub struct MexcFutures {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub web_user_token: Option<String>,
    pub client: Client
}


fn get_md5(string: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(string);
    format!("{:x}", hasher.finalize())
}

impl MexcFutures {

    pub fn new(api_key: Option<String>, api_secret: Option<String>, web_user_token: Option<String>, proxy_url: Option<String>) -> anyhow::Result<Self> {

        let client = match proxy_url {
            Some(url) => {
                let proxy = reqwest::Proxy::all(url)?;
                reqwest::Client::builder().proxy(proxy).build()?
            },
            None => reqwest::Client::new()
        };


        Ok(Self {
            api_key,
            api_secret,
            web_user_token,
            client
        })
    }

    pub fn sign_v1(&self, timestamp: u128, sign_params: Option<&str>) -> anyhow::Result<String> {


        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;
        let secret_key = self.api_secret.as_ref().ok_or_else(|| anyhow!("Missing secret key"))?;


        let sign = match sign_params {
            Some(params) => format!("{}{}{}", api_key, timestamp, params),
            None => format!("{}{}", api_key, timestamp),
        };
    
        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(sign.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }

    pub async fn ping(&self) -> anyhow::Result<Duration> {
        let url = format!("{FUTURES_API_URL}/api/v1/contract/ping");

        let inst = Instant::now();
        let _ = self.client.get(url).send().await?;

        Ok(inst.elapsed())
    }

    pub async fn get_futures_account(&self) -> anyhow::Result<Vec<FuturesBalance>> {

        let url = format!("{}/api/v1/private/account/assets", FUTURES_API_URL);

        let headers = self.generate_signed_header(None)?;

        let resp: Response = self.client.get(url).headers(headers).send().await?;

        let json_str: String = resp.text().await?;

        //println!("{json_str}");

        let resp: FuturesResponse = serde_json::from_str(&json_str)?;

        //dbg!(&resp);

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp.message);
        }

        let balances: Vec<FuturesBalance> = serde_json::from_value(resp.data.context("Expected data field")?)?;

        Ok(balances)

    }

    fn generate_signed_header(&self, sign_params: Option<&str>) -> anyhow::Result<HeaderMap> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;
        let timestamp = get_timestamp();
        let signature = self.sign_v1(timestamp, sign_params)?;
        let request_time = timestamp.to_string();

        let mut headers = HeaderMap::new();
        headers.insert("ApiKey", HeaderValue::from_str(api_key)?);
        headers.insert("Request-Time", HeaderValue::from_str(&request_time)?);
        headers.insert("Signature", HeaderValue::from_str(&signature)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        Ok(headers)
    }
    
    pub async fn get_account_asset(&self, asset: &str) -> anyhow::Result<FuturesBalance> {

        let path = format!("/api/v1/private/account/asset/{}", asset);
        let url = format!("{}{}", FUTURES_API_URL, path);

        let headers = self.generate_signed_header(None)?;

        let resp: Response = self.client.get(url).headers(headers).send().await?;

        let json_str: String = resp.text().await?;

        //println!("{json_str}");

        let resp: FuturesResponse = serde_json::from_str(&json_str)?;

        //dbg!(&resp);

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp.message);
        }

        let balance: FuturesBalance = serde_json::from_value(resp.data.context("Expected data field")?)?;

        Ok(balance)
    }

    /*
    
    Use field userToken as web user token from: https://www.mexc.com/ucenter/api/user_info
    
     */
    pub async fn submit_order(&self, symbol: &str, contract_units: u64, price: Option<f64>,leverage: u64, side: OrderDirection, open_type: OpenType, order_type: OrderType) -> anyhow::Result<OrderReceipt> {


        let web_user_token = self.web_user_token.as_ref().ok_or_else(|| anyhow!("Missing web user token"))?;

        let url = "https://futures.mexc.com/api/v1/private/order/create";

        let mut params = json!({
            "symbol": symbol,
            "side": side as u64,
            "openType": open_type as u64, 
            "type": order_type as u64, 
            "vol": contract_units,
            "leverage": leverage,
            "marketCeiling": false,
            "priceProtect": "0",
            "reduceOnly": false
        });

        if let Some(p) = price {
            params["price"] = json!(p.to_string());
        }


        let timestamp = get_timestamp().to_string();

        let partial_hash =  {
            let concat = format!("{web_user_token}{timestamp}");
            //println!("to hash: {concat}");
            get_md5(&concat).get(7..).unwrap().to_string()
        };
        //println!("partial_hash: {partial_hash}");
    
        let param_string = params.to_string();
        //println!("param_string: {param_string}");
    
        let signature = get_md5(&format!("{timestamp}{param_string}{partial_hash}"));
        //println!("signature: {signature}");
    
        let mut headers = HeaderMap::new();
            
        headers.insert("x-mxc-nonce", HeaderValue::from_str(&timestamp)?);
        headers.insert("x-mxc-sign", HeaderValue::from_str(&signature)?);
        headers.insert("authorization", HeaderValue::from_str(web_user_token)?);
        headers.insert("user-agent", HeaderValue::from_static("MEXC/7 CFNetwork/1474 Darwin/23.0.0"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("origin", HeaderValue::from_static("https://futures.mexc.com"));
        headers.insert("referer", HeaderValue::from_static("https://futures.mexc.com/exchange"));


        let resp: FuturesResponse = self.client.post(url).headers(headers).json(&params).send().await?.json().await?;

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp.message);
        }

        let receipt: OrderReceipt = serde_json::from_value(resp.data.context("Expected data field")?)?;

        Ok(receipt)
    }

    pub async fn submit_directional_orders(&self, symbol: &str, mut contract_units: u64, price: Option<f64>,leverage: u64, direction: PositionType, open_type: OpenType, order_type: OrderType) -> anyhow::Result<Vec<OrderReceipt>> {

        let open_positions = self.get_open_positions().await?;

        let positions = open_positions.iter().find(|p| p.symbol.eq(symbol) && p.leverage == leverage && p.open_type.eq(&open_type));

        let mut orders = vec![];
        let mut futures = vec![];

        if let Some(position) = positions.filter(|p| p.position_type.ne(&direction)) {

            let side = if direction.eq(&PositionType::Long) { OrderDirection::CloseShort } else { OrderDirection::CloseLong };

            if position.hold_vol >= contract_units {

                futures.push(self.submit_order(symbol, contract_units, price, leverage, side, open_type, order_type));
                contract_units = 0;

            } else {
                futures.push(self.submit_order(symbol, position.hold_vol, price, leverage, side, open_type, order_type));
                contract_units -= position.hold_vol;
            }
        } 

        if contract_units > 0 {
            let side = if direction.eq(&PositionType::Long) { OrderDirection::OpenLong } else { OrderDirection::OpenShort };
            futures.push(self.submit_order(symbol, contract_units, price, leverage, side, open_type, order_type));
        }

        let results = futures::future::join_all(futures).await;

        for result in results.into_iter() {
            orders.push(result?);
        }

        Ok(orders)
    }

    pub async fn get_open_positions(&self) -> anyhow::Result<Vec<FuturesPosition>> {

        let url = format!("{}/api/v1/private/position/open_positions", FUTURES_API_URL);

        let headers = self.generate_signed_header(None)?;

        let resp: Response = self.client.get(url).headers(headers).send().await?;

        let json_str: String = resp.text().await?;

        //println!("{json_str}");

        let resp: FuturesResponse = serde_json::from_str(&json_str)?;

        //dbg!(&resp);

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp.message);
        }

        let positions: Vec<FuturesPosition> = serde_json::from_value(resp.data.context("Expected data field")?)?;
        

        Ok(positions)
    }


    pub async fn get_fair_price(&self, symbol: &str) -> anyhow::Result<f64> {
        let url = format!("{}/api/v1/contract/index_price/{}", FUTURES_API_URL, symbol);
        let resp: FuturesResponse = self.client.get(url).send().await?.json().await?;

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp.message);
        }
        resp.data.context("Expected data field")?.get("indexPrice").context("Expected index price")?.as_f64().context("f64 convert error")
    }

    pub async fn get_contract_details(&self, symbol: &str) -> anyhow::Result<ContractInfo> {

        let url = format!("{}/api/v1/contract/detail?symbol={}", FUTURES_API_URL,symbol);

        let resp: FuturesResponse = self.client.get(url).send().await?.json().await?;

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp.message);
        }
        
        let detail: ContractInfo = serde_json::from_value(resp.data.context("Expected data field")?)?;

        Ok(detail)
    }

    pub async fn query_order(&self, order_id: &str) -> anyhow::Result<FuturesOrder> {

        let url = format!("{}/api/v1/private/order/get/{order_id}", FUTURES_API_URL);

        let headers = self.generate_signed_header(None)?;

        let resp: Response = self.client.get(url).headers(headers).send().await?;

        let json_str: String = resp.text().await?;

        let resp: FuturesResponse = serde_json::from_str(&json_str)?;

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp.message);
        }

        let order: FuturesOrder = serde_json::from_value(resp.data.context("Expected data field")?)?;
    
        Ok(order)
    }
    /* 
    // Does not work... signature verification failed
    pub async fn query_orders(&self, order_ids: Vec<String>) -> anyhow::Result<Vec<FuturesOrder>> {

        ensure!(order_ids.len() > 0, "No orders");

        let url = format!("{}/api/v1/private/order/batch_query", FUTURES_API_URL);

        let params = json!({
            "order_ids": order_ids.join(",")
        });

        let headers = self.generate_signed_header(Some(&params.to_string()))?;

        let resp: Response = self.client.get(url).headers(headers).json(&params).send().await?;

        let json_str: String = resp.text().await?;

        let resp: FuturesResponse = serde_json::from_str(&json_str)?;

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp.message);
        }

        let orders: Vec<FuturesOrder> = serde_json::from_value(resp.data.context("Expected data field")?)?;
    
        Ok(orders)
    }
    */

    /* 
    pub async fn get_all_contract_details(&self) -> anyhow::Result<()> {

        let url = format!("{}/api/v1/contract/detail", FUTURES_API_URL);

        let resp: FuturesResponse = self.client.get(url).send().await?.json().await?;

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp.message);
        }

        //let txt = resp.data.to_string();
       // println!("{txt}");
        
        let details: Vec<ContractInfo> = serde_json::from_value(resp.data.context("Expected data field")?)?;


        dbg!(&details);

        Ok(())
    }

    */

    pub fn create_websocket_login_statement(&self) -> anyhow::Result<String> {

        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;
        let timestamp = get_timestamp();
        let signature = self.sign_v1(timestamp, None)?;

        let cmd = json!({
            "method": "login",
            "param" : {
                "apiKey": api_key,
                "reqTime": timestamp.to_string(),
                "signature": signature
            }
        });
        Ok(cmd.to_string())
    }

    pub fn create_websocket_ping_statement(&self) -> String {
        json!({
            "method": "ping"
        }).to_string()
    }
}


#[cfg(test)]
mod tests {

    use crate::utils::unlock_keys;

    use super::*;


    #[tokio::test]
    pub async fn test_futures_ping() {

        let client = MexcFutures::new(None,None,None,None).unwrap();

        let dur = client.ping().await.unwrap();
        dbg!(dur);
    }

    #[tokio::test]
    pub async fn test_get_futures_account() {
        let (key, secret) = unlock_keys().unwrap();
        let client = MexcFutures::new(Some(key),Some(secret),None, None).unwrap();

        let acc = client.get_futures_account().await.unwrap();
        dbg!(acc);
    }

    #[tokio::test]
    pub async fn test_get_futures_asset_info() {
        let (key, secret) = unlock_keys().unwrap();
        let client = MexcFutures::new(Some(key),Some(secret),None, None).unwrap();

        let acc = client.get_account_asset("USDT").await.unwrap();
        dbg!(acc);
    }

    #[tokio::test]
    pub async fn test_futures_get_open_positions() {
        let (key, secret) = unlock_keys().unwrap();
        let client = MexcFutures::new(Some(key),Some(secret),None, None).unwrap();

        let acc = client.get_open_positions().await.unwrap();
        dbg!(acc);
    }

    #[tokio::test]
    pub async fn test_futures_get_fair_price() {

        let client = MexcFutures::new(None,None,None, None).unwrap();
        let p = client.get_fair_price("BTC_USDT").await.unwrap();
        dbg!(p);
    }

    #[tokio::test]
    pub async fn test_futures_get_contract_details() {

        let client = MexcFutures::new(None,None,None, None).unwrap();
        let p = client.get_contract_details("ETH_USDT").await.unwrap();
        dbg!(p);
    }

    /* 
    #[tokio::test]
    pub async fn test_futures_get_all_contract_details() {

        let client = MexcFutures::new(None,None,None, None).unwrap();
        let p = client.get_all_contract_details().await.unwrap();
        dbg!(p);
    }

    */

    #[tokio::test]
    pub async fn test_futures_submit_order() {
        let (key, secret) = unlock_keys().unwrap();

        // Powershell: $Env:web_token="xxx"
        let web_token = std::env::var("web_token").unwrap();

        let client = MexcFutures::new(Some(key),Some(secret),Some(web_token), None).unwrap();

        let symbol = "ETH_USDT";
        let q = 0.02;
        let price = None; //Some(3650.13);

        let i = client.get_contract_details(symbol).await.unwrap();

        let contract_units = (q / i.contract_size) as u64;

        println!("contract_units: {contract_units}");

        let receipt = client.submit_order(symbol, contract_units, price, 4, OrderDirection::CloseShort, OpenType::Cross, OrderType::Market).await.unwrap();
        dbg!(receipt);
    }

    #[tokio::test]
    pub async fn test_futures_submit_directional_order() {
        let (key, secret) = unlock_keys().unwrap();

        // Powershell: $Env:web_token="xxx"
        let web_token = std::env::var("web_token").unwrap();

        let client = MexcFutures::new(Some(key),Some(secret),Some(web_token), None).unwrap();

        let symbol = "ETH_USDT";
        let q = 0.02;
        let price = None; //Some(3650.13);

        let i = client.get_contract_details(symbol).await.unwrap();

        let contract_units = (q / i.contract_size) as u64;

        println!("contract_units: {contract_units}");

        let latency = Instant::now();

        let receipts = client.submit_directional_orders(symbol, contract_units, price, 4, PositionType::Short, OpenType::Cross, OrderType::Market).await.unwrap();
        dbg!(latency.elapsed()); // latency.elapsed() = 1.4562507s, latency.elapsed() = 1.1419579s with futures join all
        dbg!(receipts);
    }

    #[tokio::test]
    pub async fn test_futures_query_order() {
        let (key, secret) = unlock_keys().unwrap();
        let client = MexcFutures::new(Some(key),Some(secret),None, None).unwrap();

        // market order 575758889245571072
        // limit order 575755030422977024


        let acc = client.query_order("575758889245571072").await.unwrap();
        dbg!(acc);

        // deal columns for execute price info
        // taker_fee: 0.0049071
    }

    /*
    #[tokio::test]
    pub async fn test_futures_query_multiple_order() {
        let (key, secret) = unlock_keys().unwrap();
        let client = MexcFutures::new(Some(key),Some(secret),None, None).unwrap();

        // market order 575758889245571072
        // limit order 575755030422977024

        let orders = vec!["575758889245571072".into()];
        let acc = client.query_orders(orders).await.unwrap();
        dbg!(acc);

        // deal columns for execute price info
        // taker_fee: 0.0049071
    } */

    #[tokio::test]
    pub async fn test_futures_websocket_login() {
        let (key, secret) = unlock_keys().unwrap();
        let client = MexcFutures::new(Some(key),Some(secret),None, None).unwrap();
        let json = client.create_websocket_login_statement().unwrap();
        println!("{json}");
    }
}