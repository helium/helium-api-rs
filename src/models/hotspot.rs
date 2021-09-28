use super::Dbi;
use crate::{Error, Result};

use serde::{de, Deserialize, Serialize};
use std::{fmt, str::FromStr};

use super::Geocode;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hotspot {
    /// The address of the hotspots. This is the public key in base58
    /// check-encoding of the hotspot.
    pub address: String,
    /// The hotspot owner wallet address
    pub owner: String,
    /// The "animal" name of the hotspot. The name can be `None` for
    /// some API endpoints.
    pub name: Option<String>,
    /// The block height when the hotspot was added to the blockchain
    pub added_height: Option<u64>,
    /// The last asserted latitude of the hotspot
    pub lat: Option<f64>,
    /// The last asserted longitude of the hotspot
    pub lng: Option<f64>,
    /// The h3 index based on the lat/lon of the hotspot is used for
    /// PoC challenges.
    pub location: Option<String>,
    /// The mode in which the hotspots was added to the network.
    pub mode: HotspotStakingMode,
    /// The elevation (in meters) above or belowo sea level
    pub elevation: Option<i32>,
    /// The gain (in dbi) above or belowo sea level
    #[serde(deserialize_with = "Dbi::deserialize_option")]
    pub gain: Option<Dbi>,
    /// The geocode information for the hotspot location
    pub geocode: Geocode,
    /// The current nonce for the hotspot
    pub nonce: u64,
    /// The speculative nonce for the hotspot. This field is only meaningful
    /// when a single hotspot is requested
    #[serde(default)]
    pub speculative_nonce: u64,
    /// The current reward scale for the hotspot
    pub reward_scale: Option<f64>,
}

#[derive(Clone, Serialize, Debug)]
pub enum HotspotStakingMode {
    Full,
    Light,
    DataOnly,
}

impl fmt::Display for HotspotStakingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataOnly => f.write_str("dataonly"),
            Self::Full => f.write_str("full"),
            Self::Light => f.write_str("light"),
        }
    }
}

impl<'de> Deserialize<'de> for HotspotStakingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct HotspotStakingModeVisitor;

        impl<'de> de::Visitor<'de> for HotspotStakingModeVisitor {
            type Value = HotspotStakingMode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("full, light, dataonly")
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<HotspotStakingMode, E>
            where
                E: de::Error,
            {
                match HotspotStakingMode::from_str(value) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(de::Error::custom("invalid staking mode")),
                }
            }
        }

        deserializer.deserialize_str(HotspotStakingModeVisitor)
    }
}
impl FromStr for HotspotStakingMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_ref() {
            "light" => Ok(Self::Light),
            "full" => Ok(Self::Full),
            "dataonly" => Ok(Self::DataOnly),
            _ => Err(Error::value(s.into())),
        }
    }
}
