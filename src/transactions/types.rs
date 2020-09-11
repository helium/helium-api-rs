use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use super::custom_serde;

#[derive(PartialEq, Clone, Default)]
pub struct Signature {
    data: std::vec::Vec<u8>,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            custom_serde::base64_url::to_string(self.data.as_slice())
        )
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        custom_serde::base64_url::serialize(self.data.as_slice(), serializer)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Signature, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Signature {
            data: custom_serde::base64_url::deserialize(deserializer)?,
        })
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&custom_serde::base64_url::to_string(self.data.as_slice()))
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct Pubkey {
    data: std::vec::Vec<u8>,
}

impl Pubkey {
    pub fn from_vec(vec: std::vec::Vec<u8>) -> Pubkey {
        Pubkey {
            data: vec
        }
    }
}

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            custom_serde::base58::to_string(self.data.as_slice())
        )
    }
}

impl Serialize for Pubkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        custom_serde::base58::serialize(self.data.as_slice(), serializer)
    }
}

impl<'de> Deserialize<'de> for Pubkey {
    fn deserialize<D>(deserializer: D) -> Result<Pubkey, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Pubkey {
            data: custom_serde::base58::deserialize(deserializer)?,
        })
    }
}

impl fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&custom_serde::base58::to_string(self.data.as_slice()))
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct Hash {
    data: std::vec::Vec<u8>,
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            custom_serde::base64_url::to_string(self.data.as_slice())
        )
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        custom_serde::base64_url::serialize(self.data.as_slice(), serializer)
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Hash {
            data: custom_serde::base64_url::deserialize(deserializer)?,
        })
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&custom_serde::base64_url::to_string(self.data.as_slice()))
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct DataField {
    data: std::vec::Vec<u8>,
}

impl Serialize for DataField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        custom_serde::base64_url::serialize(self.data.as_slice(), serializer)
    }
}

impl<'de> Deserialize<'de> for DataField {
    fn deserialize<D>(deserializer: D) -> Result<DataField, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DataField {
            data: custom_serde::base64_url::deserialize(deserializer)?,
        })
    }
}

impl fmt::Debug for DataField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&custom_serde::base64_url::to_string(self.data.as_slice()))
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct Memo {
    data: std::vec::Vec<u8>,
}

impl Serialize for Memo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        custom_serde::base64::serialize(self.data.as_slice(), serializer)
    }
}

impl<'de> Deserialize<'de> for Memo {
    fn deserialize<D>(deserializer: D) -> Result<Memo, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Memo {
            data: custom_serde::base64::deserialize(deserializer)?,
        })
    }
}

impl fmt::Debug for Memo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&custom_serde::base64::to_string(self.data.as_slice()))
    }
}
