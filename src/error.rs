use thiserror::Error;
pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("request error")]
    Request(#[from] reqwest::Error),
    #[error("encode error")]
    Encode(#[from] prost::EncodeError),
    #[error("unexpected value")]
    Value(serde_json::Value),
    #[error("invalid decimals in {0}, only 8 allowed")]
    Decimals(String),
}

pub fn value(value: serde_json::Value) -> Error {
    Error::Value(value)
}

pub fn decimals(value: &str) -> Error {
    Error::Decimals(value.to_string())
}
