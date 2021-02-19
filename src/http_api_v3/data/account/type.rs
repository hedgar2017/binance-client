//!
//! The account type.
//!

use serde::Deserialize;

///
/// The account type.
///
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Spot,
    #[serde(other)]
    Other,
}
