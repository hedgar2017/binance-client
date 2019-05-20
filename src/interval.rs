//!
//! The Binance interval.
//!

use std::string::ToString;

use serde_derive::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interval {
    m1,
    m3,
    m5,
    m15,
    m30,
    h1,
    h2,
    h4,
    h6,
    h8,
    d1,
    d3,
    w1,
    M1,
}

impl Interval {
    pub fn secs(self) -> i64 {
        match self {
            Interval::m1 => 60,
            Interval::m3 => 60 * 3,
            Interval::m5 => 60 * 5,
            Interval::m15 => 60 * 15,
            Interval::m30 => 60 * 30,
            Interval::h1 => 60 * 60,
            Interval::h2 => 60 * 60 * 2,
            Interval::h4 => 60 * 60 * 4,
            Interval::h6 => 60 * 60 * 6,
            Interval::h8 => 60 * 60 * 8,
            Interval::d1 => 60 * 60 * 24,
            Interval::d3 => 60 * 60 * 24 * 3,
            Interval::w1 => 60 * 60 * 24 * 7,
            Interval::M1 => 60 * 60 * 24 * 30,
        }
    }
}

impl ToString for Interval {
    fn to_string(&self) -> String {
        match self {
            Interval::m1 => "1m",
            Interval::m3 => "3m",
            Interval::m5 => "5m",
            Interval::m15 => "15m",
            Interval::m30 => "30m",
            Interval::h1 => "1h",
            Interval::h2 => "2h",
            Interval::h4 => "4h",
            Interval::h6 => "6h",
            Interval::h8 => "8h",
            Interval::d1 => "1d",
            Interval::d3 => "3d",
            Interval::w1 => "1w",
            Interval::M1 => "1M",
        }
        .to_string()
    }
}
