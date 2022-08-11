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

#[derive(Clone, Serialize, Debug)]
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

use serde::Deserializer;

impl<'de> Deserialize<'de> for PaymentV2Payment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Type {
            Iot,
            Hnt,
            Hst,
            Mobile,
        }

        #[derive(Deserialize)]
        struct Inner {
            pub token_type: Option<Type>,
            pub amount: u64,
            pub memo: Option<String>,
            pub payee: String,
        }

        let i: Inner = Inner::deserialize(deserializer)?;

        Ok(match i.token_type {
            None => PaymentV2Payment::Hnt(HntPayment {
                amount: Hnt::from(i.amount),
                memo: i.memo,
                payee: i.payee,
            }),
            Some(tag) => match tag {
                Type::Iot => PaymentV2Payment::Iot(IotPayment {
                    amount: Iot::from(i.amount),
                    memo: i.memo,
                    payee: i.payee,
                }),
                Type::Hnt => PaymentV2Payment::Hnt(HntPayment {
                    amount: Hnt::from(i.amount),
                    memo: i.memo,
                    payee: i.payee,
                }),
                Type::Hst => PaymentV2Payment::Hst(HstPayment {
                    amount: Hst::from(i.amount),
                    memo: i.memo,
                    payee: i.payee,
                }),
                Type::Mobile => PaymentV2Payment::Mobile(MobilePayment {
                    amount: Mobile::from(i.amount),
                    memo: i.memo,
                    payee: i.payee,
                }),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn deser_json_hst() {
        let json_str = "\
        {\
            \"token_type\": \"hst\",\
            \"payee\": \"1bdkwatM2APjn5cvo7eKiJGidoHAKJsHg8ReeguH49dtLacK6cR\",\
            \"memo\": \"HNTAAAAAAAA=\",\
            \"max\": false,\
            \"amount\": 160000000\
          }\
        ";
        let hnt_payment: PaymentV2Payment =
            serde_json::from_str(json_str).expect("hst_payment deserialization");
        if let PaymentV2Payment::Hst(hst) = hnt_payment {
            assert_eq!(hst.amount, Hst::from_str("1.6").expect("HST"));
        } else {
            assert!(false)
        }
    }

    #[test]
    fn deser_json_hnt() {
        // when no token_type is given, we default to HNT
        let json_str = "\
        {\
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
