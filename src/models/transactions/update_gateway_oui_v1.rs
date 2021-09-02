use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateGatewayOuiV1 {
    pub gateway: String,
    pub hash: String,
    pub oui: u64,
    pub nonce: u64,
    pub fee: u64,
    pub gateway_owner_signature: String,
    pub oui_owner_signature: String,
}
