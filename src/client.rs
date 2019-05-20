//!
//! The Binance HTTP adapter.
//!

use crate::{
    error::Error,
    exchange::ExchangeInfo,
    interval::Interval,
    kline::{Kline, KlineInner},
};

use reqwest::{Method, Url};

pub struct Client {
    inner: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub const API_URL: &'static str = "https://api.binance.com/api/v1";

    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }

    pub fn klines(
        &self,
        interval: Interval,
        symbol: String,
        limit: Option<usize>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<Kline>, Error> {
        let mut url = format!(
            "{}/klines?interval={}&symbol={}",
            Client::API_URL,
            interval.to_string(),
            symbol,
        );
        if let Some(limit) = limit {
            url += &format!("&limit={}", limit);
        }
        if let Some(start_time) = start_time {
            url += &format!("&startTime={}", start_time);
        }
        if let Some(end_time) = end_time {
            url += &format!("&endTime={}", end_time);
        }
        let url = Url::parse(&url).map_err(Error::UrlParsing)?;

        let request = self
            .inner
            .request(Method::GET, url)
            .build()
            .map_err(Error::RequestBuilding)?;
        let result: Vec<KlineInner> = self
            .inner
            .execute(request)
            .map_err(Error::RequestExecution)?
            .json()
            .map_err(Error::RequestParsing)?;
        let klines: Vec<Kline> = result.into_iter().map(Kline::from).collect();

        Ok(klines)
    }

    pub fn exchange_info(&self) -> Result<ExchangeInfo, Error> {
        let url =
            Url::parse(&format!("{}/exchangeInfo", Client::API_URL,)).map_err(Error::UrlParsing)?;

        let request = self
            .inner
            .request(Method::GET, url)
            .build()
            .map_err(Error::RequestBuilding)?;
        let result: ExchangeInfo = self
            .inner
            .execute(request)
            .map_err(Error::RequestExecution)?
            .json()
            .map_err(Error::RequestParsing)?;

        Ok(result)
    }
}
