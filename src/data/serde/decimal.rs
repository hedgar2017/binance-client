//!
//! The decimal deserializer.
//!

use std::str::FromStr;

use rust_decimal::Decimal;

struct Visitor;

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Decimal;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Not a valid decimal")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de::{Error, Unexpected};
        Decimal::from_str(s).map_err(|_| Error::invalid_value(Unexpected::Str(s), &self))
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(deserializer.deserialize_str(Visitor)?)
}
