//!
//! The Binance HTTP adapter.
//!

pub mod error;
pub mod response;

use hmac::Hmac;
use hmac::Mac;
use hmac::NewMac;
use reqwest::Method;
use reqwest::Url;
use sha2::Sha256;

use crate::data::exchange_info::get::response::Response as ExchangeInfoGetResponse;
use crate::data::klines::get::request::Query as KlinesGetRequest;
use crate::data::klines::get::response::Response as KlinesGetResponse;
use crate::data::order::delete::request::Query as OrderDeleteRequest;
use crate::data::order::delete::response::Response as OrderDeleteResponse;
use crate::data::order::get::request::Query as OrderGetRequest;
use crate::data::order::get::response::Response as OrderGetResponse;
use crate::data::order::post::request::Query as OrderPostRequest;
use crate::data::order::post::response::Response as OrderPostResponse;
use crate::data::time::get::response::Response as TimeGetResponse;

use self::error::Error;
use self::response::Response;

///
/// The Binance HTTP client.
///
pub struct Client {
    /// The inner HTTP client.
    inner: reqwest::Client,
    /// The Binance authorization API key.
    api_key: Option<String>,
    /// The Binance authorization secret key.
    secret_key: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

type Result<T> = ::std::result::Result<T, Error>;

impl Client {
    /// The Binance API base URL.
    const BASE_URL: &'static str = "https://api.binance.com";

    ///
    /// Creates an unauthorized client instance.
    ///
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
            api_key: None,
            secret_key: None,
        }
    }

    ///
    /// Creates an authorized client instance.
    ///
    pub fn new_with_auth(api_key: String, secret_key: String) -> Self {
        Self {
            inner: reqwest::Client::new(),
            api_key: Some(api_key),
            secret_key: Some(secret_key),
        }
    }

    ///
    /// Test connectivity to the Rest API.
    ///
    pub fn ping(&self) -> Result<()> {
        self.execute::<()>(Method::GET, "/api/v3/ping".to_owned())
    }

    ///
    /// Test connectivity to the Rest API and get the current server time.
    ///
    pub fn time(&self) -> Result<TimeGetResponse> {
        self.execute::<TimeGetResponse>(Method::GET, "/api/v3/time".to_owned())
    }

    ///
    /// Current exchange trading rules and symbol information.
    ///
    pub fn exchange_info(&self) -> Result<ExchangeInfoGetResponse> {
        self.execute::<ExchangeInfoGetResponse>(Method::GET, "/api/v3/exchangeInfo".to_owned())
    }

    ///
    /// Kline/candlestick bars for a symbol.
    /// Klines are uniquely identified by their open time.
    ///
    pub fn klines(&self, request: KlinesGetRequest) -> Result<KlinesGetResponse> {
        self.execute::<KlinesGetResponse>(
            Method::GET,
            format!("/api/v3/klines?{}", request.to_string()),
        )
    }

    ///
    /// Check an order's status.
    ///
    pub fn order_get(&self, request: OrderGetRequest) -> Result<OrderGetResponse> {
        let secret_key = self
            .secret_key
            .as_ref()
            .ok_or(Error::AuthorizationKeysMissing)?;

        let mut params = request.to_string();
        params += &format!("&signature={}", Self::signature(&params, secret_key));

        self.execute_signed::<OrderGetResponse>(Method::GET, format!("/api/v3/order?{}", params))
    }

    ///
    /// Send in a new order.
    ///
    pub fn order_post(&self, request: OrderPostRequest) -> Result<OrderPostResponse> {
        let secret_key = self
            .secret_key
            .as_ref()
            .ok_or(Error::AuthorizationKeysMissing)?;

        let mut params = request.to_string();
        params += &format!("&signature={}", Self::signature(&params, secret_key));

        self.execute_signed::<OrderPostResponse>(Method::POST, format!("/api/v3/order?{}", params))
    }

    ///
    /// Cancel an active order.
    ///
    pub fn order_delete(&self, request: OrderDeleteRequest) -> Result<OrderDeleteResponse> {
        let secret_key = self
            .secret_key
            .as_ref()
            .ok_or(Error::AuthorizationKeysMissing)?;

        let mut params = request.to_string();
        params += &format!("&signature={}", Self::signature(&params, secret_key));

        self.execute_signed::<OrderDeleteResponse>(
            Method::DELETE,
            format!("/api/v3/order?{}", params),
        )
    }

    ///
    /// Test new order creation and signature/recvWindow long.
    /// Creates and validates a new order but does not send it into the matching engine.
    ///
    pub fn order_post_test(&self, request: OrderPostRequest) -> Result<OrderPostResponse> {
        let secret_key = self
            .secret_key
            .as_ref()
            .ok_or(Error::AuthorizationKeysMissing)?;

        let mut params = request.to_string();
        params += &format!("&signature={}", Self::signature(&params, secret_key));

        self.execute_signed::<OrderPostResponse>(
            Method::POST,
            format!("/api/v3/order/test?{}", params),
        )
    }

    ///
    /// Executes an unauthorized request.
    ///
    fn execute<T>(&self, method: Method, url: String) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = Self::BASE_URL.to_owned() + url.as_str();

        let response: Response<T> = self
            .inner
            .execute(
                self.inner
                    .request(
                        method,
                        Url::parse(&url).map_err(|error| Error::UrlParsing(error, url))?,
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

    ///
    /// Executes an authorized request.
    ///
    fn execute_signed<T>(&self, method: Method, url: String) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or(Error::AuthorizationKeysMissing)?;

        let url = Self::BASE_URL.to_owned() + url.as_str();

        let response: Response<T> = self
            .inner
            .execute(
                self.inner
                    .request(
                        method,
                        Url::parse(&url).map_err(|error| Error::UrlParsing(error, url))?,
                    )
                    .header("X-MBX-APIKEY", api_key.to_owned())
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

    ///
    /// Generates an HMAC signature for authorized requests.
    ///
    fn signature(params: &str, secret_key: &str) -> String {
        hex::encode(
            {
                let mut hmac: Hmac<Sha256> =
                    Hmac::new_varkey(secret_key.as_bytes()).expect(crate::panic::HMAC_ALWAYS_VALID);
                hmac.update(params.as_bytes());
                hmac.finalize().into_bytes()
            }
            .to_vec(),
        )
    }
}