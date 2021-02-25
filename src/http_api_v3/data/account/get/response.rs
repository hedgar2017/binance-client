//!
//! The account GET response.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::http_api_v3::data::account::balance::Balance;
use crate::http_api_v3::data::account::r#type::AccountType;
use crate::http_api_v3::data::permission::Permission;

///
/// The `https://www.binance.com/api/v3/account` GET response.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The maker fee.
    pub maker_commission: i64,
    /// The taker fee.
    pub taker_commission: i64,
    /// The buyer fee.
    pub buyer_commission: i64,
    /// The seller fee.
    pub seller_commission: i64,
    /// Whether the account is allowed to trade.
    pub can_trade: bool,
    /// Whether the account is allowed to withdraw.
    pub can_withdraw: bool,
    /// Whether the account is allowed to deposit.
    pub can_deposit: bool,
    /// The account last update time.
    pub update_time: i64,
    /// The account type.
    pub account_type: AccountType,
    /// The account balances.
    pub balances: Vec<Balance>,
    /// The account permissions.
    pub permissions: Vec<Permission>,
}

impl Response {
    ///
    /// Get the available balance for the specified token.
    ///
    pub fn get_balance(&self, token: &str) -> Decimal {
        self.balances
            .iter()
            .find(|balance| balance.asset.as_str() == token)
            .map(|balance| balance.free)
            .unwrap_or_default()
    }
}
