use serde::{Deserialize, Serialize};

mod account;
mod block;
mod geocode;
mod hotspot;
mod oracle;
mod oui;
mod pending_transaction;
pub mod transactions;
mod validator;
mod values;

pub use account::*;
pub use block::*;
pub use geocode::*;
pub use hotspot::*;
pub use oracle::*;
pub use oui::*;
pub use pending_transaction::*;
pub use validator::*;
pub use values::*;

/// Query params for requests that can take in a time range.
#[derive(Clone, Debug, Serialize)]
pub struct QueryTimeRange {
    /// ISO 8601 timestamp or relative time (-3 hour) minimum time range
    pub min_time: Option<String>,
    /// ISO 8601 timestamp or relative time (-3 hour) maximum time range
    pub max_time: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct QueryFilterWithTimeRange {
    /// ISO 8601 timestamp or relative time (-3 hour) minimum time range
    pub min_time: Option<String>,
    /// ISO 8601 timestamp or relative time (-3 hour) maximum time range
    pub max_time: Option<String>,
    pub filter_types: Option<String>,
    pub limit: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct QueryFilter {
    pub filter_types: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct QueryLimitWithTimeRange {
    /// ISO 8601 timestamp or relative time (-3 hour) minimum time range
    pub min_time: Option<String>,
    /// ISO 8601 timestamp or relative time (-3 hour) maximum time range
    pub max_time: Option<String>,
    pub limit: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BucketType {
    Hour,
    Day,
    Week,
}

#[derive(Clone, Debug, Serialize)]
pub struct QueryBucketWithTimeRange {
    /// ISO 8601 timestamp or relative time (-3 hour) minimum time range
    pub min_time: Option<String>,
    /// ISO 8601 timestamp or relative time (-3 hour) maximum time range
    pub max_time: Option<String>,
    pub bucket: Option<BucketType>,
}
