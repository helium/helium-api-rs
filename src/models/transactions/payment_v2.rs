use crate::models::{Hnt, Hst, Iot, Mobile};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaymentV2 {
    pub hash: String,
    /// Fee is in datacredits
    pub fee: u64,
    pub nonce: u64,
    pub payer: String,
    pub payments: Vec<PaymentV2Payment>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "token_type", rename_all = "snake_case")]
pub enum PaymentV2Payment {
    Iot(IotPayment),
    Hnt(HntPayment),
    Hst(HstPayment),
    Mobile(MobilePayment),
}

macro_rules! payment {
    ($payment_type:ident, $token:ident) => {
        #[derive(Clone, Serialize, Deserialize, Debug)]
        pub struct $payment_type {
            pub amount: $token,
            pub memo: Option<String>,
            pub payee: String,
        }
    };
}
payment!(HstPayment, Hst);
payment!(IotPayment, Iot);
payment!(HntPayment, Hnt);
payment!(MobilePayment, Mobile);

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn deser_json_hnt() {
        let json_str = "\
        {\
            \"token_type\": \"hnt\",\
            \"payee\": \"1bdkwatM2APjn5cvo7eKiJGidoHAKJsHg8ReeguH49dtLacK6cR\",\
            \"memo\": \"HNTAAAAAAAA=\",\
            \"max\": false,\
            \"amount\": 160000000\
          }\
        ";
        let hnt_payment: PaymentV2Payment =
            serde_json::from_str(json_str).expect("hnt_payment deserialization");
        if let PaymentV2Payment::Hnt(hnt) = hnt_payment {
            assert_eq!(hnt.amount, Hnt::from_str("1.6").expect("HNT"));
        } else {
            assert!(false)
        }
    }
}
