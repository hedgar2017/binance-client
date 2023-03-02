//!
//! The products GET request.
//!

///
/// The `https://www.binance.com/exchange-api/v2/public/asset-service/product/get-products` GET request query.
///
pub struct Query {
    /// Whether to include ETF.
    pub include_etf: bool,
}

impl Query {
    /// The query params default capacity.
    const QUERY_INITIAL_CAPACITY: usize = 256;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(include_etf: bool) -> Self {
        Self { include_etf }
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(Self::QUERY_INITIAL_CAPACITY);
        params += &format!("includeEtf={}", self.include_etf);
        params
    }
}
