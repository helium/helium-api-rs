use serde::Deserialize;

#[derive(Deserialize)]
pub struct Height {
    /// The current block height of the chain.
    pub height: u64,
}
