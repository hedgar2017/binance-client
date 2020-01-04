//!
//! The Binance client binary.
//!

use failure::Fail;
use log::*;

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
    info!("{:?}", info);

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "binance_client=info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
}
