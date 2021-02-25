//!
//! The Binance product request example.
//!

use binance_client::ProductBySymbolGetQuery;

static SYMBOL: &str = "BTCUSDT";

///
/// The Binance product request example.
///
fn main() {
    let client = binance_client::HttpExchangeApiV2Client::new();

    let symbol = client
        .get_product_by_symbol(ProductBySymbolGetQuery::new(SYMBOL.to_owned()))
        .expect("Product request");

    println!(
        "{} status: {:?}",
        SYMBOL,
        symbol.data.map(|symbol| symbol.status)
    );
}
