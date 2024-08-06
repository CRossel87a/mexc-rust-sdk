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
    pub data: Option<Value>,
    pub message: Option<String>
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


#[derive(Deserialize, Debug)]
pub struct ContractInfo {
    #[serde(rename = "amountScale")]
    pub amount_scale: i32,

    #[serde(rename = "apiAllowed")]
    pub api_allowed: bool,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub appraisal: f64,

    #[serde(rename = "askLimitPriceRate", deserialize_with = "parse_string_to_f64")]
    pub ask_limit_price_rate: f64,

    #[serde(rename = "automaticDelivery")]
    pub automatic_delivery: i32,

    #[serde(rename = "baseCoin")]
    pub base_coin: String,

    #[serde(rename = "baseCoinIconUrl")]
    pub base_coin_icon_url: String,

    #[serde(rename = "baseCoinId")]
    pub base_coin_id: String,

    #[serde(rename = "baseCoinName")]
    pub base_coin_name: String,

    #[serde(rename = "bidLimitPriceRate", deserialize_with = "parse_string_to_f64")]
    pub bid_limit_price_rate: f64,

    #[serde(rename = "conceptPlate")]
    pub concept_plate: Vec<String>,

    #[serde(rename = "contractSize", deserialize_with = "parse_string_to_f64")]
    pub contract_size: f64,

    #[serde(rename = "depthStepList")]
    pub depth_step_list: Vec<String>,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "displayNameEn")]
    pub display_name_en: String,

    #[serde(rename = "futureType")]
    pub future_type: i32,

    pub id: i64,

    #[serde(rename = "indexOrigin")]
    pub index_origin: Vec<String>,

    #[serde(rename = "initialMarginRate", deserialize_with = "parse_string_to_f64")]
    pub initial_margin_rate: f64,

    #[serde(rename = "isHidden")]
    pub is_hidden: bool,

    #[serde(rename = "isHot")]
    pub is_hot: bool,

    #[serde(rename = "isNew")]
    pub is_new: bool,

    #[serde(rename = "limitMaxVol", deserialize_with = "parse_string_to_f64")]
    pub limit_max_vol: f64,

    #[serde(rename = "maintenanceMarginRate", deserialize_with = "parse_string_to_f64")]
    pub maintenance_margin_rate: f64,

    #[serde(rename = "makerFeeRate", deserialize_with = "parse_string_to_f64")]
    pub maker_fee_rate: f64,

    #[serde(rename = "marketOrderMaxLevel")]
    pub market_order_max_level: i32,

    #[serde(rename = "marketOrderPriceLimitRate1", deserialize_with = "parse_string_to_f64")]
    pub market_order_price_limit_rate1: f64,

    #[serde(rename = "marketOrderPriceLimitRate2", deserialize_with = "parse_string_to_f64")]
    pub market_order_price_limit_rate2: f64,

    #[serde(rename = "maxLeverage")]
    pub max_leverage: i32,

    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: Vec<i32>,

    #[serde(rename = "maxVol", deserialize_with = "parse_string_to_f64")]
    pub max_vol: f64,

    #[serde(rename = "minLeverage")]
    pub min_leverage: i32,

    #[serde(rename = "minVol", deserialize_with = "parse_string_to_f64")]
    pub min_vol: f64,

    #[serde(rename = "positionOpenType")]
    pub position_open_type: i32,

    #[serde(rename = "priceCoefficientVariation", deserialize_with = "parse_string_to_f64")]
    pub price_coefficient_variation: f64,

    #[serde(rename = "priceScale")]
    pub price_scale: i32,

    #[serde(rename = "priceUnit", deserialize_with = "parse_string_to_f64")]
    pub price_unit: f64,

    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,

    #[serde(rename = "quoteCoinName")]
    pub quote_coin_name: String,

    #[serde(rename = "riskBaseVol", deserialize_with = "parse_string_to_f64")]
    pub risk_base_vol: f64,

    #[serde(rename = "riskIncrImr", deserialize_with = "parse_string_to_f64")]
    pub risk_incr_imr: f64,

    #[serde(rename = "riskIncrMmr", deserialize_with = "parse_string_to_f64")]
    pub risk_incr_mmr: f64,

    #[serde(rename = "riskIncrVol", deserialize_with = "parse_string_to_f64")]
    pub risk_incr_vol: f64,

    #[serde(rename = "riskLevelLimit")]
    pub risk_level_limit: i32,

    #[serde(rename = "riskLimitType")]
    pub risk_limit_type: String,

    #[serde(rename = "riskLongShortSwitch")]
    pub risk_long_short_switch: i32,

    #[serde(rename = "settleCoin")]
    pub settle_coin: String,

    #[serde(rename = "showAppraisalCountdown")]
    pub show_appraisal_countdown: i32,

    pub state: i32,

    pub symbol: String,

    #[serde(rename = "takerFeeRate", deserialize_with = "parse_string_to_f64")]
    pub taker_fee_rate: f64,

    #[serde(deserialize_with = "parse_string_to_f64")]
    pub threshold: f64,

    #[serde(rename = "triggerProtect", deserialize_with = "parse_string_to_f64")]
    pub trigger_protect: f64,

    pub vid: String,

    #[serde(rename = "volScale")]
    pub vol_scale: i32,

    #[serde(rename = "volUnit", deserialize_with = "parse_string_to_f64")]
    pub vol_unit: f64,
}

#[derive(Deserialize, Debug)]
pub struct OrderReceipt {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "ts")]
    pub timestamp: u128
}