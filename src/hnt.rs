use crate::error;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde::Serialize;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Hnt(Decimal);

const HNT_TO_BONES_SCALAR: i32 = 100_000_000;

impl FromStr for Hnt {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = Decimal::from_str(s)
            .or_else(|_| Decimal::from_scientific(s))
            .unwrap();

        if data.scale() > 8 {
            Err(error::decimals(s))
        } else {
            Ok(Hnt(data))
        }
    }
}

impl Hnt {
    pub fn to_bones(&self) -> u64 {
        if let Some(scaled_dec) = self.0.checked_mul(HNT_TO_BONES_SCALAR.into()) {
            if let Some(num) = scaled_dec.to_u64() {
                return num;
            }
        }
        panic!("Hnt has been constructed with invalid data")
    }

    pub fn from_bones(bones: u64) -> Self {
        if let Some(mut data) = Decimal::from_u64(bones) {
            data.set_scale(8).unwrap();
            return Hnt(data);
        }
        panic!("Bones value could not be parsed into Decimal")
    }

    pub fn get_decimal(&self) -> Decimal {
        self.0
    }
}

impl ToString for Hnt {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
