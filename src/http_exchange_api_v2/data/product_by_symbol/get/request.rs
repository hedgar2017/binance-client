//!
//! The product by symbol GET request.
//!

///
/// The `https://www.binance.com/exchange-api/v2/public/asset-service/product/get-product-by-symbol` GET request query.
///
pub struct Query {
    /// The symbol name.
    pub symbol: String,
}

impl Query {
    /// The query params default capacity.
    const QUERY_INITIAL_CAPACITY: usize = 256;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(Self::QUERY_INITIAL_CAPACITY);
        params += &format!("symbol={}", self.symbol);
        params
    }
}
