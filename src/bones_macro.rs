#[macro_export]
macro_rules! decimal_bones {
    ( $t:tt  , $s:ident) => {
        use crate::error;
        use rust_decimal::prelude::*;
        use rust_decimal::Decimal;
        use std::str::FromStr;

        impl FromStr for $t {
            type Err = error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let data = Decimal::from_str(s)
                    .or_else(|_| Decimal::from_scientific(s))
                    .unwrap();

                if data.scale() > 8 {
                    Err(error::decimals(s))
                } else {
                    Ok($t(data))
                }
            }
        }

        impl $t {
            pub fn to_bones(&self) -> u64 {
                if let Some(scaled_dec) = self.0.checked_mul($s.into()) {
                    if let Some(num) = scaled_dec.to_u64() {
                        return num;
                    }
                }
                panic!("Type has been constructed with invalid data")
            }

            pub fn from_bones(bones: u64) -> Self {
                if let Some(mut data) = Decimal::from_u64(bones) {
                    data.set_scale(8).unwrap();
                    return $t(data);
                }
                panic!("Value could not be parsed into Decimal")
            }

            pub fn get_decimal(&self) -> Decimal {
                self.0
            }
        }
    };
}
