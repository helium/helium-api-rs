use crate::{Error, Result};
use core::fmt;
use rust_decimal::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

macro_rules! decimal_scalar {
    ($stype:ident, $scalar:literal, $scale:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Default)]
        pub struct $stype(Decimal);

        impl FromStr for $stype {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                match Decimal::from_str(s).or_else(|_| Decimal::from_scientific(s)) {
                    Ok(data) if data.scale() > 8 => Err(Error::decimals(s)),
                    Ok(data) => Ok(Self(data)),
                    Err(_) => Err(Error::decimals(s)),
                }
            }
        }

        impl Serialize for $stype {
            fn serialize<S>(&self, s: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let u: u64 = u64::from(*self);
                s.serialize_u64(u)
            }
        }

        impl<'de> Deserialize<'de> for $stype {
            fn deserialize<D>(d: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let val = u64::deserialize(d)?;
                Ok(Self::from(val))
            }
        }

        impl fmt::Display for $stype {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl $stype {
            pub fn new(d: Decimal) -> Self {
                Self(d)
            }

            pub fn get_decimal(&self) -> Decimal {
                self.0
            }

            pub fn deserialize_option<'de, D>(d: D) -> std::result::Result<Option<Self>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let v: Option<u64> = Option::deserialize(d)?;
                if let Some(val) = v {
                    Ok(Some(Self::from(val)))
                } else {
                    Ok(None)
                }
            }
        }

        impl From<u64> for $stype {
            fn from(v: u64) -> Self {
                if let Some(mut data) = Decimal::from_u64(v) {
                    data.set_scale($scale).unwrap();
                    return Self(data);
                }
                panic!("u64 could not be converted into Decimal")
            }
        }

        impl From<$stype> for u64 {
            fn from(v: $stype) -> Self {
                if let Some(scaled_dec) = v.0.checked_mul($scalar.into()) {
                    if let Some(num) = scaled_dec.to_u64() {
                        return num;
                    }
                }
                panic!("Invalid scaled decimal construction")
            }
        }

        impl From<i32> for $stype {
            fn from(v: i32) -> Self {
                if let Some(mut data) = Decimal::from_i32(v) {
                    data.set_scale($scale).unwrap();
                    return Self(data);
                }
                panic!("u64 could not be converted into Decimal")
            }
        }

        impl From<$stype> for i32 {
            fn from(v: $stype) -> Self {
                if let Some(scaled_dec) = v.0.checked_mul($scalar.into()) {
                    if let Some(num) = scaled_dec.to_i32() {
                        return num;
                    }
                }
                panic!("Invalid scaled decimal construction")
            }
        }
    };
}

decimal_scalar!(Hnt, 100_000_000, 8);
decimal_scalar!(Hst, 100_000_000, 8);
decimal_scalar!(Iot, 100_000_000, 8);
decimal_scalar!(Mobile, 100_000_000, 8);
decimal_scalar!(Token, 100_000_000, 8);
decimal_scalar!(Usd, 100_000_000, 8);
decimal_scalar!(Dbi, 10, 1);

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_ser_hnt() {
        let hnt = Hnt::from(5500);

        assert_tokens(&hnt, &[Token::U64(5500)]);
    }
}
