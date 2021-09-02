use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RoutingV1 {
    pub fee: u64,
    pub oui: u64,
    pub hash: String,
    pub nonce: u64,
    pub owner: String,
    pub action: RoutingAction,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum RoutingAction {
    NewXor(NewXor),
    UpdateXor(UpdateXor),
    UpdateRouters(UpdateRouters),
    RequestSubnet(RequestSubnet),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewXor {
    pub filter: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateXor {
    pub filter: String,
    pub index: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateRouters {
    pub addresses: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RequestSubnet {
    pub requested_subnet_size: u64,
}
