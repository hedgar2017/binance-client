//!
//! The order status.
//!

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
}

impl Status {
    pub fn is_partially_filled(self) -> bool {
        match self {
            Status::PartiallyFilled => true,
            _ => false,
        }
    }

    pub fn is_filled(self) -> bool {
        match self {
            Status::Filled => true,
            _ => false,
        }
    }
}
