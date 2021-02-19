//!
//! The exchange info symbol.
//!

pub mod filter;
pub mod status;

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::http_api_v3::data::order_type::OrderType;
use crate::http_api_v3::data::permission::Permission;

use self::filter::Filter;
use self::status::Status;

///
/// The trading symbol data.
///
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    /// The symbol name.
    pub symbol: String,
    /// The symbol status on the exchange.
    pub status: Status,
    /// The secondary token in the trading pair.
    pub base_asset: String,
    /// The generic number of fractional digits in the secondary token.
    /// Do not use for the price scale!
    pub base_asset_precision: usize,
    /// The primary token in the trading pair.
    pub quote_asset: String,
    /// The generic number of fractional digits in the primary token.
    /// Do not use for the price scale!
    pub quote_precision: usize,
    /// The order types allowed for the symbol.
    pub order_types: Vec<OrderType>,
    /// If iceberd order is allowed for the symbol.
    pub iceberg_allowed: bool,
    /// The conditions Binance puts on the symbol.
    pub filters: Vec<Filter>,
    /// The allowed trading methods like spot, margin, etc.
    pub permissions: Vec<Permission>,
}

impl Symbol {
    ///
    /// If the symbol is active and can be normally traded.
    ///
    pub fn is_trading(&self) -> bool {
        matches!(self.status, Status::Trading)
    }

    ///
    /// If margin trading is allowed for the symbol.
    ///
    pub fn has_margin(&self) -> bool {
        self.permissions.contains(&Permission::Margin)
    }

    ///
    /// The number of fractional digits in the symbol price.
    ///
    /// E.g. `0.000256` has `6` fractional digits.
    /// E.g. `0.42` has `2` fractional digits.
    ///
    pub fn price_precision(&self) -> Option<u32> {
        for filter in self.filters.iter() {
            if let Filter::PriceFilter { tick_size, .. } = filter {
                let mut scale = crate::r#const::PRECISION_DEFAULT;
                let mut tick_size_check = Decimal::new(1, scale);
                while *tick_size > tick_size_check && scale > 0 {
                    tick_size_check *= Decimal::new(10, 0);
                    scale -= 1;
                }
                return Some(scale);
            }
        }
        None
    }

    ///
    /// The number of fractional digits in the symbol quantity.
    ///
    pub fn quantity_precision(&self) -> Option<u32> {
        for filter in self.filters.iter() {
            if let Filter::LotSize { step_size, .. } = filter {
                let mut scale = crate::r#const::PRECISION_DEFAULT;
                let mut step_size_check = Decimal::new(1, scale);
                while *step_size > step_size_check && scale > 0 {
                    step_size_check *= Decimal::new(10, 0);
                    scale -= 1;
                }
                return Some(scale);
            }
        }
        None
    }
}
