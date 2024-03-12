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

pub use error::{Error, Result};

pub mod accounts;
pub mod blocks;
pub mod hotspots;
pub mod models;
pub mod oracle;
pub mod ouis;
pub mod pending_transactions;
pub mod stats;
pub mod transactions;
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

impl Client {
    /// Create a new client using a given base URL and a default
    /// timeout. The library will use absoluate paths based on this
    /// base_url.
    pub fn new_with_base_url(base_url: String, user_agent: &str) -> Self {
        Self::new_with_timeout(base_url, user_agent, DEFAULT_TIMEOUT)
    }

    /// Create a new client using a given base URL, and request
    /// timeout value.  The library will use absoluate paths based on
    /// the given base_url.
    pub fn new_with_timeout(base_url: String, user_agent: &str, timeout: u64) -> Self {
        let client = reqwest::Client::builder()
            .gzip(true)
            .user_agent(user_agent)
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
                                    //loop until we find next bit of data or run
                                    // out of cursors
                                    let mut data: Data<Vec<E>>;
                                    let mut cursor = cursor;
                                    loop {
                                        data = client
                                            .fetch_data::<Vec<E>, _>(&path, &[("cursor", &cursor)])
                                            .await?;

                                        if !data.data.is_empty() {
                                            data.data.reverse();
                                            let entry = data.data.pop().unwrap();
                                            break Ok(Some((entry, (data, client, path))));
                                        } else if data.cursor.is_none() {
                                            break Ok(None);
                                        }
                                        cursor = data.cursor.unwrap();
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

impl<T: ?Sized> IntoVec for T where T: StdStream {}

#[async_trait]
pub trait IntoVec: StreamExt {
    async fn into_vec<T>(self) -> Result<Vec<T>>
    where
        Self: Sized,
        T: std::marker::Send,
        Vec<Result<T>>: Extend<Self::Item>,
    {
        self.collect::<Vec<Result<T>>>().await.into_iter().collect()
    }
}

#[cfg(test)]
fn get_test_client() -> Client {
    use std::{env, thread, time};
    const USER_AGENT: &str = "helium-api-test/0.1.0";
    const BASE_URL: &str = "https://api.helium.io/v1";
    let duration = time::Duration::from_millis(env::var("TEST_DELAY_MS").map_or(0, |v| {
        v.parse::<u64>()
            .expect("TEST_DELAY_MS cannot be parsed as u64")
    }));
    thread::sleep(duration);
    Client::new_with_base_url(BASE_URL.into(), USER_AGENT)
}
