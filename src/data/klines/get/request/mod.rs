//!
//! The klines GET request.
//!

use crate::data::Interval;

pub struct Request {
    pub symbol: String,
    pub interval: Interval,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<usize>,
}

impl Request {
    pub fn new(
        symbol: String,
        interval: Interval,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            symbol,
            interval,
            start_time,
            end_time,
            limit,
        }
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(128);
        params += &format!("symbol={}", self.symbol.to_owned());
        params += &format!("&interval={}", self.interval.to_string());
        if let Some(start_time) = self.start_time {
            params += &format!("&startTime={}", start_time);
        }
        if let Some(end_time) = self.end_time {
            params += &format!("&endTime={}", end_time);
        }
        if let Some(limit) = self.limit {
            params += &format!("&limit={}", limit);
        }
        params
    }
}
