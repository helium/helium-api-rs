use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GenGatewayV1 {
    pub hash: String,
    pub nonce: u64,
    pub owner: String,
    pub gateway: String,
    pub location: String,
}
