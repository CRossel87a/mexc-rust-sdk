pub mod utils;
pub mod market;
pub mod orders;
pub mod testing;
pub mod account;
pub mod futures;

use std::time::{Duration, Instant};
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
    pub timestamp: u128
}

impl Mexc {

    pub fn new(api_key: Option<String>, api_secret: Option<String>, proxy_url: Option<String>) -> anyhow::Result<Self> {

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
            client
        })
    }

    pub async fn get_server_time(&self) -> anyhow::Result<u128> {
        let url = format!("{PROD_API_URL}/api/v3/time");
        let resp = self.client.get(url).send().await?;

        let st: ServerTime = resp.json().await?;
        Ok(st.timestamp)
    }

    pub async fn ping(&self) -> anyhow::Result<Duration> {
        let url = format!("{PROD_API_URL}/api/v3/ping");

        let inst = Instant::now();
        let _ = self.client.get(url).send().await?;

        Ok(inst.elapsed())
    }
}