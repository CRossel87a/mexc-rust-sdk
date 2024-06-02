use serde::Deserializer;
use serde::de::{self, Visitor, SeqAccess};
use std::fmt;
use crate::Level;
use serde::Deserialize;

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