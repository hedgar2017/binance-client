//!
//! The depth GET request.
//!

///
/// The `https://www.binance.com/api/v3/depth` GET request query.
///
pub struct Query {
    /// The symbol name.
    pub symbol: String,
    /// The maximum number of depth elements to return.
    pub limit: Option<i64>,
}

impl Query {
    /// The query params default capacity.
    const QUERY_INITIAL_CAPACITY: usize = 256;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(symbol: &str, limit: Option<i64>) -> Self {
        Self {
            symbol: symbol.to_owned(),
            limit,
        }
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(Self::QUERY_INITIAL_CAPACITY);
        params += &format!("symbol={}", self.symbol.to_owned());
        if let Some(limit) = self.limit {
            params += &format!("&limit={}", limit);
        }
        params
    }
}
