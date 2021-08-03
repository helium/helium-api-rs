use serde::Serialize;

mod account;
mod block;
mod geocode;
mod hotspot;
mod oracle;
mod oui;
mod transaction;
mod validator;
mod values;

pub use account::*;
pub use block::*;
pub use geocode::*;
pub use hotspot::*;
pub use oracle::*;
pub use oui::*;
pub use transaction::{PendingTxnStatus, Transaction};
pub use validator::*;
pub use values::*;

/// Query params for requests that can take in a time range.
#[derive(Clone, Debug, Serialize)]
pub struct QueryTimeRange {
    /// ISO 8601 timestamp or relative time (-3 hour) minimum time range
    pub min_time: String,
    /// ISO 8601 timestamp or relative time (-3 hour) maximum time range
    pub max_time: String,
}
