//!
//! The order POST response type.
//!

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Ack,
    Result,
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
