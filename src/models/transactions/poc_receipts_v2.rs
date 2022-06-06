use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PocReceiptsV2 {
    pub hash: String,
    pub challenger: String,
    pub fee: u64,
    pub onion_key_hash: String,
    pub path: Vec<PathElement>,
    pub challenger_owner: String,
    pub secret: String,
    pub time: u64,
    pub height: u64,
    pub block_hash: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PathElement {
    pub challengee: String,
    pub challengee_owner: String,
    pub challengee_lat: f64,
    pub challengee_lon: f64,
    pub challengee_location_hex: String,
    pub challengee_location: String,
    pub receipt: Option<Receipt>,
    pub geocode: Option<Geocode>,
    pub witnesses: Vec<Witness>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Geocode {
    pub short_street: Option<String>,
    pub short_state: Option<String>,
    pub short_country: Option<String>,
    pub short_city: Option<String>,
    pub long_street: Option<String>,
    pub long_state: Option<String>,
    pub long_country: Option<String>,
    pub long_city: Option<String>,
    pub city_id: Option<String>,
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
