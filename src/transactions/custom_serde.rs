pub mod base58 {
    extern crate bs58;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn to_string(bytes: &[u8]) -> String {
        bs58::encode(bytes).into_string()
    }

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&to_string(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        bs58::decode(s).into_vec().map_err(de::Error::custom)
    }
}

pub mod base64_url {
    extern crate base64;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn to_string(bytes: &[u8]) -> String {
        base64::encode_config(bytes, base64::URL_SAFE)
    }

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&to_string(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        base64::decode_config(s, base64::URL_SAFE).map_err(de::Error::custom)
    }
}

pub mod base64 {
    extern crate base64;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn to_string(bytes: &[u8]) -> String {
        base64::encode(bytes)
    }

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&to_string(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        base64::decode(s).map_err(de::Error::custom)
    }
}
