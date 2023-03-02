//!
//! The exchange info symbol filter.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

///
/// The exchange info symbol filter.
///
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "filterType")]
pub enum Filter {
    /// The corresponding filter variant.
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        /// The minimal price.
        min_price: Decimal,
        /// The maximum price.
        max_price: Decimal,
        /// The tick size.
        tick_size: Decimal,
    },
    /// The corresponding filter variant.
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        /// The multiplier up.
        multiplier_up: Decimal,
        /// The multiplier down.
        multiplier_down: Decimal,
        /// The average price minimums.
        avg_price_mins: i64,
    },
    /// The corresponding filter variant.
    #[serde(rename_all = "camelCase")]
    LotSize {
        /// The minimal quantity.
        min_qty: Decimal,
        /// The maximum price.
        max_qty: Decimal,
        /// The step size.
        step_size: Decimal,
    },
    /// The corresponding filter variant.
    #[serde(rename_all = "camelCase")]
    MinNotional {
        /// The notional minimum.
        min_notional: Decimal,
        /// If apply to market.
        apply_to_market: bool,
        /// The average price minimums.
        avg_price_mins: i64,
    },
    /// The corresponding filter variant.
    #[serde(rename_all = "camelCase")]
    IcebergParts {
        /// The iceberg limit.
        limit: i64,
    },
    /// The corresponding filter variant.
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        /// The minimal quantity.
        min_qty: Decimal,
        /// The maximum price.
        max_qty: Decimal,
        /// The step size.
        step_size: Decimal,
    },
    /// The corresponding filter variant.
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders {
        /// The maximum number of orders.
        max_num_algo_orders: i64,
    },
    /// The corresponding filter variant.
    #[serde(rename_all = "camelCase")]
    MaxNumOrders {
        /// The maximum number of orders.
        max_num_orders: i64,
    },
    /// Other variants.
    #[serde(other)]
    Other,
}
