//!
//! The Binance time request example.
//!

use chrono::prelude::*;

///
/// The Binance time request example.
///
fn main() {
    let client = binance_client::HttpApiV3Client::new();

    let system_time = Utc::now().timestamp_millis();
    let request_time = std::time::Instant::now();
    let binance_time = client.time().expect("Time request").server_time
        - (request_time.elapsed().as_millis() as i64) / 2;

    println!("Binance : {}", binance_time);
    println!("System  : {}", system_time);
    println!("Diff    : {}ms", system_time - binance_time);
}
