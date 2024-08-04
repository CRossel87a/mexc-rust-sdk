use serde::Deserialize;
use serde_json::Value;
use crate::utils::parse_string_to_f64;


#[derive(Deserialize, Debug)]
pub struct FuturesBalance {
    pub currency: String,

    #[serde(rename = "positionMargin")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub position_margin: f64,

    #[serde(rename = "availableBalance")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub available_balance: f64,

    #[serde(rename = "cashBalance")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub cash_balance: f64,

    #[serde(rename = "frozenBalance")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub frozen_balance: f64,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub equity: f64,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub unrealized: f64,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub bonus: f64,
}


#[derive(Deserialize, Debug)]
pub struct FuturesResponse {
    pub success: bool,
    pub code: i64,
    pub data: Value
}

#[derive(Deserialize, Debug)]
pub struct FuturesPosition {
    #[serde(rename = "autoAddIm")]
    pub auto_add_im: bool,

    #[serde(rename = "closeAvgPrice", deserialize_with = "parse_string_to_f64")]
    pub close_avg_price: f64,

    #[serde(rename = "closeProfitLoss", deserialize_with = "parse_string_to_f64")]
    pub close_profit_loss: f64,

    #[serde(rename = "closeVol", deserialize_with = "parse_string_to_f64")]
    pub close_vol: f64,

    #[serde(rename = "createTime")]
    pub create_time: u128,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub fee: f64,

    #[serde(rename = "frozenVol", deserialize_with = "parse_string_to_f64")]
    pub frozen_vol: f64,

    #[serde(rename = "holdAvgPrice", deserialize_with = "parse_string_to_f64")]
    pub hold_avg_price: f64,

    #[serde(rename = "holdAvgPriceFullyScale", deserialize_with = "parse_string_to_f64")]
    pub hold_avg_price_fully_scale: f64,

    #[serde(rename = "holdFee", deserialize_with = "parse_string_to_f64")]
    pub hold_fee: f64,

    #[serde(rename = "holdVol", deserialize_with = "parse_string_to_f64")]
    pub hold_vol: f64,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub im: f64,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub leverage: f64,

    #[serde(rename = "liquidatePrice", deserialize_with = "parse_string_to_f64")]
    pub liquidate_price: f64,

    #[serde(rename = "marginRatio", deserialize_with = "parse_string_to_f64")]
    pub margin_ratio: f64,

    #[serde(rename = "newCloseAvgPrice", deserialize_with = "parse_string_to_f64")]
    pub new_close_avg_price: f64,

    #[serde(rename = "newOpenAvgPrice", deserialize_with = "parse_string_to_f64")]
    pub new_open_avg_price: f64,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub oim: f64,

    #[serde(rename = "openAvgPrice", deserialize_with = "parse_string_to_f64")]
    pub open_avg_price: f64,

    #[serde(rename = "openAvgPriceFullyScale", deserialize_with = "parse_string_to_f64")]
    pub open_avg_price_fully_scale: f64,

    #[serde(rename = "openType")]
    pub open_type: i32,

    #[serde(rename = "positionId")]
    pub position_id: i64,

    #[serde(rename = "positionType")]
    pub position_type: i32,

    #[serde(rename = "profitRatio", deserialize_with = "parse_string_to_f64")]
    pub profit_ratio: f64,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub realised: f64,

    pub state: i32,

    pub symbol: String,

    #[serde(rename = "updateTime")]
    pub update_time: u128,

    pub version: i32,
}