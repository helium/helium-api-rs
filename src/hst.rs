use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Hst(Decimal);

const HST_TO_BONES_SCALAR: i32 = 100_000_000;

impl FromStr for Hst {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = Decimal::from_str(s)
            .or_else(|_| Decimal::from_scientific(s))
            .unwrap();

        if data.scale() > 8 {
            Err(format!(
                "Too many decimals in input {}. Only 8 decimals permitted",
                s
            )
            .into())
        } else {
            Ok(Hst(data))
        }
    }
}

impl Hst {
    pub fn to_bones(&self) -> u64 {
        if let Some(scaled_dec) = self.0.checked_mul(HST_TO_BONES_SCALAR.into()) {
            if let Some(num) = scaled_dec.to_u64() {
                return num;
            }
        }
        panic!("Hst has been constructed with invalid data")
    }

    pub fn from_bones(bones: u64) -> Self {
        if let Some(mut data) = Decimal::from_u64(bones) {
            data.set_scale(8).unwrap();
            return Hst(data);
        }
        panic!("Bones value could not be parsed into Decimal")
    }

    pub fn get_decimal(&self) -> Decimal {
        self.0
    }
}

impl ToString for Hst {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
