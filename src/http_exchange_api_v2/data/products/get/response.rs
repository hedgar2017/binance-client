//!
//! The products GET response.
//!

use crate::http_exchange_api_v2::data::symbol::Symbol;

///
/// The `https://www.binance.com/exchange-api/v2/public/asset-service/product/get-products` GET response.
///
pub type Response = Vec<Symbol>;
