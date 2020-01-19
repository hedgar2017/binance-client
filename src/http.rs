//!
//! The HTTP adapter.
//!

use failure::Fail;
use hmac::Hmac;
use hmac::Mac;
use reqwest::Method;
use reqwest::Url;
use serde_derive::Deserialize;
use sha2::Sha256;

use crate::data;

pub struct Client {
    inner: reqwest::Client,
    api_key: Option<String>,
    secret_key: Option<String>,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "URL parsing: {}", _0)]
    UrlParsing(reqwest::UrlError),
    #[fail(display = "Request building: {}", _0)]
    RequestBuilding(reqwest::Error),
    #[fail(display = "Request execution: {}", _0)]
    RequestExecution(reqwest::Error),
    #[fail(display = "Response parsing: {}", _0)]
    ResponseParsing(reqwest::Error),
    #[fail(display = "Response error: {:?}", _0)]
    ResponseError(data::ResponseError),
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<O> {
    Ok(O),
    Error(data::ResponseError),
}

type Result<O> = std::result::Result<O, Error>;

impl Client {
    const ADDRESS: &'static str = "https://api.binance.com";

    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
            api_key: None,
            secret_key: None,
        }
    }

    pub fn new_with_auth(api_key: String, secret_key: String) -> Self {
        Self {
            inner: reqwest::Client::new(),
            api_key: Some(api_key),
            secret_key: Some(secret_key),
        }
    }

    pub fn ping(&self) -> Result<()> {
        self.execute::<()>(Method::GET, &format!("{}/api/v1/ping", Self::ADDRESS))
    }

    pub fn time(&self) -> Result<data::TimeGetResponse> {
        self.execute::<data::TimeGetResponse>(
            Method::GET,
            &format!("{}/api/v1/time", Self::ADDRESS),
        )
    }

    pub fn exchange_info(&self) -> Result<data::ExchangeInfoGetResponse> {
        self.execute::<data::ExchangeInfoGetResponse>(
            Method::GET,
            &format!("{}/api/v1/exchangeInfo", Self::ADDRESS),
        )
    }

    pub fn klines(&self, request: data::KlinesGetRequest) -> Result<data::KlinesGetResponse> {
        self.execute::<data::KlinesGetResponse>(
            Method::GET,
            &format!("{}/api/v1/klines?{}", Self::ADDRESS, request.to_string()),
        )
    }

    pub fn order_get(&self, request: data::OrderGetRequest) -> Result<data::OrderGetResponse> {
        let mut params = request.to_string();
        params += &format!("&signature={}", self.signature(&params));

        self.execute_signed::<data::OrderGetResponse>(
            Method::GET,
            &format!("{}/api/v3/order?{}", Self::ADDRESS, params),
        )
    }

    pub fn order_post(&self, request: data::OrderPostRequest) -> Result<data::OrderPostResponse> {
        let mut params = request.to_string();
        params += &format!("&signature={}", self.signature(&params));

        self.execute_signed::<data::OrderPostResponse>(
            Method::POST,
            &format!("{}/api/v3/order?{}", Self::ADDRESS, params),
        )
    }

    pub fn order_delete(
        &self,
        request: data::OrderDeleteRequest,
    ) -> Result<data::OrderDeleteResponse> {
        let mut params = request.to_string();
        params += &format!("&signature={}", self.signature(&params));

        self.execute_signed::<data::OrderDeleteResponse>(
            Method::DELETE,
            &format!("{}/api/v3/order?{}", Self::ADDRESS, params),
        )
    }

    pub fn order_post_test(
        &self,
        request: data::OrderPostRequest,
    ) -> Result<data::OrderPostResponse> {
        let mut params = request.to_string();
        params += &format!("&signature={}", self.signature(&params));

        self.execute_signed::<data::OrderPostResponse>(
            Method::POST,
            &format!("{}/api/v3/order/test?{}", Self::ADDRESS, params),
        )
    }

    fn execute<O>(&self, method: Method, url: &str) -> Result<O>
    where
        O: serde::de::DeserializeOwned,
    {
        let response: Response<O> = self
            .inner
            .execute(
                self.inner
                    .request(method, Url::parse(url).map_err(Error::UrlParsing)?)
                    .build()
                    .map_err(Error::RequestBuilding)?,
            )
            .map_err(Error::RequestExecution)?
            .json()
            .map_err(Error::ResponseParsing)?;

        match response {
            Response::Ok(response) => Ok(response),
            Response::Error(error) => Err(Error::ResponseError(error)),
        }
    }

    fn execute_signed<O>(&self, method: Method, url: &str) -> Result<O>
    where
        O: serde::de::DeserializeOwned,
    {
        let response: Response<O> = self
            .inner
            .execute(
                self.inner
                    .request(method, Url::parse(url).map_err(Error::UrlParsing)?)
                    .header(
                        "X-MBX-APIKEY",
                        self.api_key.to_owned().expect("API key is None"),
                    )
                    .build()
                    .map_err(Error::RequestBuilding)?,
            )
            .map_err(Error::RequestExecution)?
            .json()
            .map_err(Error::ResponseParsing)?;

        match response {
            Response::Ok(response) => Ok(response),
            Response::Error(error) => Err(Error::ResponseError(error)),
        }
    }

    fn signature(&self, params: &str) -> String {
        hex::encode(
            {
                let mut hmac: Hmac<Sha256> = Hmac::new_varkey(
                    self.secret_key
                        .as_ref()
                        .expect("Auth not provided")
                        .as_bytes(),
                )
                .expect("HMAC creation bug");
                hmac.input(params.as_bytes());
                hmac.result().code()
            }
            .to_vec(),
        )
    }
}
