//!
//! The exchange info symbol filter.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "filterType")]
pub enum Filter {
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: Decimal,
        max_price: Decimal,
        tick_size: Decimal,
    },
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: Decimal,
        multiplier_down: Decimal,
        avg_price_mins: i64,
    },
    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },
    #[serde(rename_all = "camelCase")]
    MinNotional {
        min_notional: Decimal,
        apply_to_market: bool,
        avg_price_mins: i64,
    },
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: i64 },
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: i64 },
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: i64 },
    #[serde(other)]
    Other,
}
