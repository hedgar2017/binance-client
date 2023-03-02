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
    #[serde(rename = "1m")]
    Minute1,
    /// The 3 minutes interval.
    #[serde(rename = "3m")]
    Minute3,
    /// The 5 minutes interval.
    #[serde(rename = "5m")]
    Minute5,
    /// The 15 minutes interval.
    #[serde(rename = "15m")]
    Minute15,
    /// The 30 minutes interval.
    #[serde(rename = "30m")]
    Minute30,
    /// The 1 hour interval.
    #[serde(rename = "1H")]
    Hour1,
    /// The 2 hours interval.
    #[serde(rename = "2H")]
    Hour2,
    /// The 4 hours interval.
    #[serde(rename = "4H")]
    Hour4,
    /// The 6 hours interval.
    #[serde(rename = "6H")]
    Hour6,
    /// The 8 hours interval.
    #[serde(rename = "8H")]
    Hour8,
    /// The 12 hours interval.
    #[serde(rename = "12H")]
    Hour12,
    /// The 1 day interval.
    #[serde(rename = "1D")]
    Day1,
    /// The 3 days interval.
    #[serde(rename = "3D")]
    Day3,
    /// The 1 week interval.
    #[serde(rename = "1W")]
    Week1,
    /// The 1 month interval.
    #[serde(rename = "1M")]
    Month1,
}

impl From<Interval> for i64 {
    fn from(value: Interval) -> i64 {
        match value {
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
                Interval::Hour12 => "12h",
                Interval::Day1 => "1d",
                Interval::Day3 => "3d",
                Interval::Week1 => "1w",
                Interval::Month1 => "1M",
            }
        )
    }
}
