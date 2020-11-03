//!
//! The order status.
//!

use serde::Deserialize;

///
/// The order status.
///
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order is just created.
    New,
    /// The order is partially filled.
    PartiallyFilled,
    /// The order is completely filled.
    Filled,
    /// The order is cancelled.
    Canceled,
    /// The order is requested for cancel.
    PendingCancel,
    /// The order is rejected.
    Rejected,
    /// The order is expired.
    Expired,
}

impl OrderStatus {
    ///
    /// A shortcut predicate.
    ///
    pub fn is_partially_filled(self) -> bool {
        matches!(self, OrderStatus::PartiallyFilled)
    }

    ///
    /// A shortcut predicate.
    ///
    pub fn is_filled(self) -> bool {
        matches!(self, OrderStatus::Filled)
    }
}
