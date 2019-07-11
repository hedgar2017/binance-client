//!
//! The response error.
//!

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    code: i64,
    msg: String,
}
