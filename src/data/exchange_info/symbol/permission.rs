//!
//! The exchange info symbol permission.
//!

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    Spot,
    Margin,
    #[serde(other)]
    Other,
}
