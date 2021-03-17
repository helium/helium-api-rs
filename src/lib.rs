use async_trait::async_trait;
use futures::{
    future, stream, Future as StdFuture, FutureExt, Stream as StdStream, StreamExt, TryFutureExt,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{pin::Pin, time::Duration};

/// A type alias for `Future` that may return `crate::error::Error`
pub type Future<T> = Pin<Box<dyn StdFuture<Output = Result<T>> + Send>>;

/// A type alias for `Stream` that may result in `crate::error::Error`
pub type Stream<T> = Pin<Box<dyn StdStream<Item = Result<T>> + Send>>;

mod error;
mod values;

pub use error::{Error, Result};
pub use values::{Hnt, Hst, Usd};
pub mod accounts;
pub mod blocks;
pub mod hotspots;
pub mod oracle;
pub mod ouis;
pub mod pending_transactions;
pub mod validators;
pub mod vars;

/// The default timeout for API requests
pub const DEFAULT_TIMEOUT: u64 = 120;
/// The default base URL if none is specified.
pub const DEFAULT_BASE_URL: &str = "https://api.helium.io/v1";
/// A utility constant to pass an empty query slice to the various client fetch
/// functions
pub const NO_QUERY: &[&str; 0] = &[""; 0];

#[derive(Clone, Deserialize, Debug)]
pub(crate) struct Data<T> {
    pub data: T,
    pub cursor: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Default for Client {
    /// Create a new client using the hosted Helium API at
    /// explorer.helium.foundation
    fn default() -> Self {
        Self::new_with_base_url(DEFAULT_BASE_URL.to_string())
    }
}

impl Client {
    /// Create a new client using a given base URL and a default
    /// timeout. The library will use absoluate paths based on this
    /// base_url.
    pub fn new_with_base_url(base_url: String) -> Self {
        Self::new_with_timeout(base_url, DEFAULT_TIMEOUT)
    }

    /// Create a new client using a given base URL, and request
    /// timeout value.  The library will use absoluate paths based on
    /// the given base_url.
    pub fn new_with_timeout(base_url: String, timeout: u64) -> Self {
        let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap();
        Self { base_url, client }
    }

    pub(crate) fn fetch_data<T, Q>(&self, path: &str, query: &Q) -> Future<Data<T>>
    where
        T: 'static + DeserializeOwned + std::marker::Send,
        Q: Serialize + ?Sized,
    {
        let request_url = format!("{}{}", self.base_url, path);
        self.client
            .get(&request_url)
            .query(query)
            .send()
            .map_err(error::Error::from)
            .and_then(|response| match response.error_for_status() {
                Ok(result) => {
                    let data: Future<Data<T>> = result.json().map_err(error::Error::from).boxed();
                    data
                }
                Err(e) => future::err(error::Error::from(e)).boxed(),
            })
            .boxed()
    }

    pub(crate) fn fetch_stream<E, Q>(&self, path: &str, query: &Q) -> Stream<E>
    where
        E: 'static + DeserializeOwned + std::marker::Send,
        Q: Serialize + ?Sized,
    {
        let path = path.to_string();
        let client = self.clone();
        client
            .fetch_data::<Vec<E>, _>(&path, query)
            .map_ok(move |mut data| {
                data.data.reverse();
                stream::try_unfold(
                    (data, client, path),
                    |(mut data, client, path)| async move {
                        match data.data.pop() {
                            Some(entry) => Ok(Some((entry, (data, client, path)))),
                            None => match data.cursor {
                                Some(cursor) => {
                                    let mut data = client
                                        .fetch_data::<Vec<E>, _>(&path, &[("cursor", &cursor)])
                                        .await?;
                                    data.data.reverse();
                                    match data.data.pop() {
                                        Some(entry) => Ok(Some((entry, (data, client, path)))),
                                        None => Ok(None),
                                    }
                                }
                                None => Ok(None),
                            },
                        }
                    },
                )
            })
            .try_flatten_stream()
            .boxed()
    }

    pub(crate) async fn fetch<T, Q>(&self, path: &str, query: &Q) -> error::Result<T>
    where
        T: 'static + DeserializeOwned + std::marker::Send,
        Q: Serialize + ?Sized,
    {
        let result = self.fetch_data(path, query).await?;
        Ok(result.data)
    }

    pub(crate) fn post<T, R>(&self, path: &str, json: &T) -> Future<R>
    where
        T: Serialize + ?Sized,
        R: 'static + DeserializeOwned + std::marker::Send,
    {
        let request_url = format!("{}{}", self.base_url, path);
        self.client
            .post(&request_url)
            .json(json)
            .send()
            .map_err(error::Error::from)
            .and_then(|response| match response.error_for_status() {
                Ok(result) => {
                    let data: Future<R> = result
                        .json()
                        .map_err(error::Error::from)
                        .map_ok(|v: Data<R>| v.data)
                        .boxed();
                    data
                }
                Err(e) => future::err(error::Error::from(e)).boxed(),
            })
            .boxed()
    }
}

#[async_trait]
pub trait IntoVec {
    type Item;

    async fn into_vec(self) -> Result<Vec<Self::Item>>;
}

#[async_trait]
impl<T> IntoVec for Stream<T>
where
    T: std::marker::Send,
{
    type Item = T;
    async fn into_vec(self) -> Result<Vec<Self::Item>> {
        self.collect::<Vec<error::Result<Self::Item>>>()
            .await
            .into_iter()
            .collect()
    }
}
