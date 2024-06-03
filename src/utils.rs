use serde::Deserializer;
use serde::Serializer;
use serde::de::{self, Visitor};
use std::fmt;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn get_timestamp() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}


pub fn parse_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringToF64Visitor;

    impl<'de> Visitor<'de> for StringToF64Visitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string that can be parsed to a f64")
        }

        fn visit_str<E>(self, value: &str) -> Result<f64, E>
        where
            E: de::Error,
        {
            value.parse::<f64>().map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_str(StringToF64Visitor)
}

pub fn serialize_f64_as_string<S>(x: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&x.to_string())
}


pub fn unlock_keys() -> anyhow::Result<(String, String)>{
    let key: String = env::var("mexcn_accesskey")?;
    let secret: String = env::var("mexn_secretkey")?;
    Ok((key, secret))
}

pub fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).floor() / y
}