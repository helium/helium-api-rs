use super::decimal_bones;

const ORACLE_PRICE_TO_BONES_SCALAR: i32 = 100_000_000;
#[derive(Clone, Copy, Debug)]
pub struct OraclePrice(Decimal);

decimal_bones!(OraclePrice, ORACLE_PRICE_TO_BONES_SCALAR);

pub mod deserializer {
    use super::OraclePrice;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OraclePrice, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<u64> = Option::deserialize(deserializer)?;
        if let Some(s) = opt {
            Ok(OraclePrice::from_bones(s))
        } else {
            panic!("Unexpected user of Oracle Desrializer")
        }
    }
}
