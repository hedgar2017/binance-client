//!
//! The Binance Exchange API v2 HTTP client.
//!

pub mod data;
pub mod response;

use reqwest::Method;
use reqwest::Url;

use crate::error::Error;

use self::data::product_by_symbol::get::request::Query as ProductBySymbolGetQuery;
use self::data::product_by_symbol::get::response::Response as ProductBySymbolGetResponse;
use self::data::products::get::request::Query as ProductsGetQuery;
use self::data::products::get::response::Response as ProductsGetResponse;
use self::response::Response;

///
/// The Binance Exchange API v2 HTTP client.
///
#[derive(Debug, Clone)]
pub struct Client {
    /// The inner HTTP client.
    inner: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

type Result<T> = ::std::result::Result<Response<T>, Error>;

impl Client {
    /// The API base URL.
    const BASE_URL: &'static str = "https://binance.com";

    ///
    /// Creates a client instance.
    ///
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }

    ///
    /// The exchange info with all known trading symbols.
    ///
    pub fn get_products(&self, request: ProductsGetQuery) -> Result<ProductsGetResponse> {
        self.execute::<ProductsGetResponse>(
            Method::GET,
            format!(
                "/exchange-api/v2/public/asset-service/product/get-products?{}",
                request.to_string()
            ),
        )
    }

    ///
    /// The single trading symbol.
    ///
    pub fn get_product_by_symbol(
        &self,
        request: ProductBySymbolGetQuery,
    ) -> Result<ProductBySymbolGetResponse> {
        self.execute::<ProductBySymbolGetResponse>(
            Method::GET,
            format!(
                "/exchange-api/v2/public/asset-service/product/get-product-by-symbol?{}",
                request.to_string()
            ),
        )
    }

    ///
    /// Executes a request.
    ///
    fn execute<T>(&self, method: Method, url: String) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = Self::BASE_URL.to_owned() + url.as_str();

        let response = self
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
            .text()
            .map_err(Error::ResponseReading)?;
        let response: Response<T> = serde_json::from_str(response.as_str())
            .map_err(|error| Error::ResponseParsing(error, response))?;

        Ok(response)
    }
}
