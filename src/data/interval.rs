//!
//! The interval.
//!

use std::fmt;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interval {
    #[serde(rename = "m1")]
    Minute1,
    #[serde(rename = "m3")]
    Minute3,
    #[serde(rename = "m5")]
    Minute5,
    #[serde(rename = "m15")]
    Minute15,
    #[serde(rename = "m30")]
    Minute30,
    #[serde(rename = "h1")]
    Hour1,
    #[serde(rename = "h2")]
    Hour2,
    #[serde(rename = "h4")]
    Hour4,
    #[serde(rename = "h6")]
    Hour6,
    #[serde(rename = "h8")]
    Hour8,
    #[serde(rename = "d1")]
    Day1,
    #[serde(rename = "d3")]
    Day3,
    #[serde(rename = "w1")]
    Week1,
    #[serde(rename = "M1")]
    Month1,
}

impl Interval {
    const SECONDS_IN_A_MINUTE: i64 = 60;
    const SECONDS_IN_AN_HOUR: i64 = Self::SECONDS_IN_A_MINUTE * 60;
    const SECONDS_IN_A_DAY: i64 = Self::SECONDS_IN_AN_HOUR * 24;

    pub fn secs(self) -> i64 {
        match self {
            Interval::Minute1 => Self::SECONDS_IN_A_MINUTE,
            Interval::Minute3 => Self::SECONDS_IN_A_MINUTE * 3,
            Interval::Minute5 => Self::SECONDS_IN_A_MINUTE * 5,
            Interval::Minute15 => Self::SECONDS_IN_A_MINUTE * 15,
            Interval::Minute30 => Self::SECONDS_IN_A_MINUTE * 30,
            Interval::Hour1 => Self::SECONDS_IN_AN_HOUR,
            Interval::Hour2 => Self::SECONDS_IN_AN_HOUR * 2,
            Interval::Hour4 => Self::SECONDS_IN_AN_HOUR * 4,
            Interval::Hour6 => Self::SECONDS_IN_AN_HOUR * 6,
            Interval::Hour8 => Self::SECONDS_IN_AN_HOUR * 8,
            Interval::Day1 => Self::SECONDS_IN_A_DAY,
            Interval::Day3 => Self::SECONDS_IN_A_DAY * 3,
            Interval::Week1 => Self::SECONDS_IN_A_DAY * 7,
            Interval::Month1 => Self::SECONDS_IN_A_DAY * 30,
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Interval::Minute1 => "1m",
                Interval::Minute3 => "3m",
                Interval::Minute5 => "5m",
                Interval::Minute15 => "15m",
                Interval::Minute30 => "30m",
                Interval::Hour1 => "1h",
                Interval::Hour2 => "2h",
                Interval::Hour4 => "4h",
                Interval::Hour6 => "6h",
                Interval::Hour8 => "8h",
                Interval::Day1 => "1d",
                Interval::Day3 => "3d",
                Interval::Week1 => "1w",
                Interval::Month1 => "1M",
            }
        )
    }
}
