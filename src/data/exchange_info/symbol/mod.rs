//!
//! The exchange info symbol.
//!

mod filter;
mod permission;
mod status;

pub use self::filter::Filter;
pub use self::permission::Permission;
pub use self::status::Status;

use rust_decimal::Decimal;
use serde_derive::Deserialize;

use crate::data::order::Type as OrderType;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: Status,
    pub base_asset: String,
    pub base_asset_precision: usize,
    pub quote_asset: String,
    pub quote_precision: usize,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub filters: Vec<Filter>,
    pub permissions: Vec<Permission>,
}

impl Symbol {
    pub fn is_trading(&self) -> bool {
        match self.status {
            Status::Trading => true,
            _ => false,
        }
    }

    pub fn has_margin(&self) -> bool {
        self.permissions.contains(&Permission::Margin)
    }

    pub fn price_scale(&self) -> Option<u32> {
        for filter in self.filters.iter() {
            if let Filter::PriceFilter { tick_size, .. } = filter {
                let mut scale = 8;
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

    pub fn quantity_scale(&self) -> Option<u32> {
        for filter in self.filters.iter() {
            if let Filter::LotSize { step_size, .. } = filter {
                let mut scale = 8;
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
