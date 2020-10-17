//!
//! The order POST response type.
//!

///
/// The `https://www.binance.com/api/v3/order` POST response type.
///
#[derive(Debug, Clone, Copy)]
pub enum Type {
    /// The ack-type. See the `ack` module.
    Ack,
    /// The result-type. See the `result` module.
    Result,
    /// The full-type. See the `full` module.
    Full,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Ack => "ACK",
            Type::Result => "RESULT",
            Type::Full => "FULL",
        }
        .to_owned()
    }
}
