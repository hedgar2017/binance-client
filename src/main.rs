//!
//! The Binance client binary.
//!

use std::env;

use failure::Fail;

use binance_client::HttpClient;

#[derive(Fail, Debug)]
enum Error {
    #[fail(display = "HTTP: {}", _0)]
    Http(binance_client::HttpError),
}

fn main() -> Result<(), Error> {
    init_logger();

    let http = HttpClient::new_with_auth(
        "k1z5VNXUfX2VKKd6seRfvsxOSfNEcwEyLvoHSHu6mCGE0AMYXo7aKfgSDBbtIUN0".to_owned(),
        "McRldEK59Enkelx9WNJgNZXjYBir990am9bJbwmdDx1kRjd4zfclUVYhbuiPpYCW".to_owned(),
    );
    let info = http.exchange_info().map_err(Error::Http)?;
    for symbol in info
        .symbols
        .into_iter()
        .filter(|symbol| symbol.price_scale().is_some() && symbol.quantity_scale().is_some())
    {
        if symbol.quote_asset != "BTC" {
            continue;
        }
        if let (Some(price_scale), Some(quantity_scale)) =
            (symbol.price_scale(), symbol.quantity_scale())
        {
            log::info!("[{}] {}/{}", symbol.symbol, price_scale, quantity_scale);
        }
    }

    Ok(())
}

fn init_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
}
