use serde::Deserialize;
use std::fmt;

#[derive(Clone, Deserialize, Debug)]
/// Represents an OUI on the blockchain
pub struct Oui {
    /// The oui value.
    pub oui: u64,
    /// The base58 public key of the owner of the oui.
    pub owner: String,
    /// The current nonce for the oui
    pub nonce: u64,
    /// The base58 encoded public keys of the routers for this oui
    pub addresses: Vec<String>,
    /// The subnets for this oui
    pub subnets: Vec<Subnet>,
}

/// Stats for ouis
#[derive(Clone, Deserialize, Debug)]
pub struct OuiStats {
    pub count: u64,
}

#[derive(Clone, Deserialize, Debug)]
/// An OUI owns a list of subnets, which are used to check if packets from a
/// device with a given DevAddr need to be sent to the routers in the OUI
pub struct Subnet {
    base: u32,
    mask: u32,
}

impl fmt::Display for Subnet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.base, self.mask))
    }
}
