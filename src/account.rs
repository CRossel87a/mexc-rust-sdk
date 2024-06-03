use crate::{Mexc, PROD_API_URL};
use reqwest::{StatusCode, Response};
use anyhow::{anyhow, bail};
use serde::Deserialize;
use crate::utils::get_timestamp;
use crate::utils::unlock_keys;
use crate::utils::parse_string_to_f64;

#[derive(Deserialize, Debug, Clone)]
pub struct Account {
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    pub permissions: Vec<String>,
    pub balances: Vec<AccountBalance>
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountBalance {
    pub asset: String,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub free: f64,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub locked: f64
}

impl Mexc {

    pub async fn get_signed(&self, url: &str) -> anyhow::Result<Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;

        let resp = self.client
        .get(url)
        .header("X-MEXC-APIKEY", api_key)
        .send().await?;
        Ok(resp)
    }

    pub async fn get_account(&self) -> anyhow::Result<Account> {

        let timestamp = get_timestamp();

        let order_request = format!("timestamp={timestamp}");
        let signed_order = self.sign_request(order_request)?;
        let url = format!("{PROD_API_URL}/api/v3/account?{signed_order}");
        let resp: Response = self.get_signed(&url).await?;

        if resp.status() == StatusCode::OK {
            let account: Account = resp.json().await?;
            Ok(account)
        } else {
            let err = resp.text().await?;
            bail!(err);
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    pub async fn test_get_account() {
        let (key, secret) = unlock_keys().unwrap();
        let client = Mexc::new(Some(key),Some(secret),None).unwrap();

        let acc = client.get_account().await.unwrap();
        dbg!(acc);
    }
}