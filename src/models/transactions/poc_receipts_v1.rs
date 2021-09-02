use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PocReceiptsV1 {
    pub hash: String,
    pub challenger: String,
    pub fee: u64,
    pub onion_key_hash: String,
    pub path: Vec<PathElement>,
    pub request_block_hash: String,
    pub secret: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PathElement {
    pub challengee: String,
    pub receipt: Option<Receipt>,
    pub witnesses: Vec<Witness>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Receipt {
    pub channel: u8,
    pub data: String,
    pub datarate: Option<String>,
    pub frequency: f64,
    pub gateway: String,
    pub origin: String,
    pub signal: i64,
    pub snr: f64,
    pub timestamp: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Witness {
    pub channel: u8,
    pub datarate: String,
    pub frequency: f64,
    pub gateway: String,
    pub is_valid: Option<bool>,
    pub packet_hash: String,
    pub signal: i64,
    pub snr: f64,
    pub timestamp: u64,
}
