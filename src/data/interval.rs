//!
//! The interval.
//!

use std::convert::TryFrom;
use std::fmt;

use serde::Deserialize;
use serde::Serialize;

///
/// The chart timeframe interval.
///
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interval {
    /// The 1 minute interval.
    #[serde(rename = "m1")]
    Minute1,
    /// The 3 minutes interval.
    #[serde(rename = "m3")]
    Minute3,
    /// The 5 minutes interval.
    #[serde(rename = "m5")]
    Minute5,
    /// The 15 minutes interval.
    #[serde(rename = "m15")]
    Minute15,
    /// The 30 minutes interval.
    #[serde(rename = "m30")]
    Minute30,
    /// The 1 hour interval.
    #[serde(rename = "H1")]
    Hour1,
    /// The 2 hours interval.
    #[serde(rename = "H2")]
    Hour2,
    /// The 4 hours interval.
    #[serde(rename = "H4")]
    Hour4,
    /// The 6 hours interval.
    #[serde(rename = "H6")]
    Hour6,
    /// The 8 hours interval.
    #[serde(rename = "H8")]
    Hour8,
    /// The 12 hours interval.
    #[serde(rename = "H12")]
    Hour12,
    /// The 1 day interval.
    #[serde(rename = "D1")]
    Day1,
    /// The 3 days interval.
    #[serde(rename = "D3")]
    Day3,
    /// The 1 week interval.
    #[serde(rename = "W1")]
    Week1,
    /// The 1 month interval.
    #[serde(rename = "M1")]
    Month1,
}

impl Into<i64> for Interval {
    fn into(self) -> i64 {
        match self {
            Interval::Minute1 => 60,
            Interval::Minute3 => 180,
            Interval::Minute5 => 300,
            Interval::Minute15 => 900,
            Interval::Minute30 => 1800,
            Interval::Hour1 => 3600,
            Interval::Hour2 => 7200,
            Interval::Hour4 => 14400,
            Interval::Hour6 => 21600,
            Interval::Hour8 => 28800,
            Interval::Hour12 => 43200,
            Interval::Day1 => 86400,
            Interval::Day3 => 259200,
            Interval::Week1 => 604800,
            Interval::Month1 => 2592000,
        }
    }
}

impl TryFrom<i64> for Interval {
    type Error = i64;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            60 => Self::Minute1,
            180 => Self::Minute3,
            300 => Self::Minute5,
            900 => Self::Minute15,
            1800 => Self::Minute30,
            3600 => Self::Hour1,
            7200 => Self::Hour2,
            14400 => Self::Hour4,
            21600 => Self::Hour6,
            28800 => Self::Hour8,
            43200 => Self::Hour12,
            86400 => Self::Day1,
            259200 => Self::Day3,
            604800 => Self::Week1,
            2592000 => Self::Month1,
            value => return Err(value),
        })
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Interval::Minute1 => "m1",
                Interval::Minute3 => "m3",
                Interval::Minute5 => "m5",
                Interval::Minute15 => "m15",
                Interval::Minute30 => "m30",
                Interval::Hour1 => "H1",
                Interval::Hour2 => "H2",
                Interval::Hour4 => "H4",
                Interval::Hour6 => "H6",
                Interval::Hour8 => "H8",
                Interval::Hour12 => "H12",
                Interval::Day1 => "D1",
                Interval::Day3 => "D3",
                Interval::Week1 => "W1",
                Interval::Month1 => "M1",
            }
        )
    }
}
