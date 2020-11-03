//!
//! The klines GET request.
//!

use crate::data::interval::Interval;

///
/// The `https://www.binance.com/api/v3/klines` GET request query.
///
pub struct Query {
    /// The symbol name.
    pub symbol: String,
    /// The timeframe interval.
    pub interval: Interval,
    /// The left time boundary of the requested klines.
    pub start_time: Option<i64>,
    /// The right time boundary of the requested klines.
    pub end_time: Option<i64>,
    /// The maximum number of klines to get.
    pub limit: Option<usize>,
}

impl Query {
    /// The query params default capacity.
    const QUERY_INITIAL_CAPACITY: usize = 128;

    ///
    /// A shortcut constructor.
    ///
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

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(Self::QUERY_INITIAL_CAPACITY);
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
