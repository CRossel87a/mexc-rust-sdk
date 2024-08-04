pub mod structures;

use hmac::{Hmac, Mac};
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

        let headers = self.generate_signed_header()?;

        let resp: Response = self.client.get(url).headers(headers).send().await?;

        let json_str: String = resp.text().await?;

        //println!("{json_str}");

        let resp: FuturesResponse = serde_json::from_str(&json_str)?;

        //dbg!(&resp);

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp);
        }

        let balances: Vec<FuturesBalance> = serde_json::from_value(resp.data)?;

        Ok(balances)

    }

    fn generate_signed_header(&self) -> anyhow::Result<HeaderMap> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;
        let timestamp = get_timestamp();
        let signature = self.sign_v1(timestamp, None)?;
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

        let headers = self.generate_signed_header()?;

        let resp: Response = self.client.get(url).headers(headers).send().await?;

        let json_str: String = resp.text().await?;

        //println!("{json_str}");

        let resp: FuturesResponse = serde_json::from_str(&json_str)?;

        //dbg!(&resp);

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp);
        }

        let balance: FuturesBalance = serde_json::from_value(resp.data)?;

        Ok(balance)
    }

    pub async fn submit_order(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn get_open_positions(&self) -> anyhow::Result<Vec<FuturesPosition>> {

        let url = format!("{}/api/v1/private/position/open_positions", FUTURES_API_URL);

        let headers = self.generate_signed_header()?;

        let resp: Response = self.client.get(url).headers(headers).send().await?;

        let json_str: String = resp.text().await?;

        //println!("{json_str}");

        let resp: FuturesResponse = serde_json::from_str(&json_str)?;

        //dbg!(&resp);

        if !resp.success {
            bail!("mexc futures err resp: {:?}", resp);
        }

        let positions: Vec<FuturesPosition> = serde_json::from_value(resp.data)?;
        

        Ok(positions)

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
    pub async fn test_futures_submit_order() {
        let (key, secret) = unlock_keys().unwrap();
        let client = MexcFutures::new(Some(key),Some(secret),None, None).unwrap();

        let acc = client.submit_order().await.unwrap();
        dbg!(acc);
    }

    #[tokio::test]
    pub async fn test_futures_get_open_positions() {
        let (key, secret) = unlock_keys().unwrap();
        let client = MexcFutures::new(Some(key),Some(secret),None, None).unwrap();

        let acc = client.get_open_positions().await.unwrap();
        dbg!(acc);
    }
}