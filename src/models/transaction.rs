use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug)]
pub struct PendingTxnStatus {
    pub hash: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaymentV2Payment {
    pub amount: u64,
    pub memo: Option<String>,
    pub payee: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Witness {
    pub channel: u8,
    pub datarate: String,
    pub frequency: f64,
    pub gateway: String,
    pub is_valid: bool,
    pub packet_hash: String,
    pub signal: i64,
    pub snr: f64,
    pub timestamp: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Receipt {
    pub channel: u8,
    pub data: String,
    pub datarate: Option<String>,
    pub frequency: f64,
    pub gateway: String,
    pub origin: String,
    pub signal: i64,
    pub snr: f64,
    pub timestamp: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PathElement {
    pub challengee: String,
    pub receipt: Option<Receipt>,
    pub witnesses: Vec<Witness>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reward {
    pub account: Option<String>,
    pub amount: u64,
    pub gateway: Option<String>,
    pub r#type: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RoutingAction {
    pub index: u64,
    pub action: String,
    pub filter: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
/// Represents a transaction response from blockchain-node.
pub enum Transaction {
    ConsensusGroupV1 {
        delay: u64,
        hash: String,
        height: u64,
        members: Vec<String>,
        proof: String,
    },
    PocRequestV1 {
        hash: String,
        block_hash: String,
        challenger: String,
        fee: u64,
        onion_key_hash: String,
        secret_hash: String,
        version: u64,
    },
    PaymentV2 {
        hash: String,
        fee: u64,
        nonce: u64,
        payer: String,
        payments: Vec<PaymentV2Payment>,
    },
    PocReceiptsV1 {
        hash: String,
        challenger: String,
        fee: u64,
        onion_key_hash: String,
        path: Vec<PathElement>,
        request_block_hash: String,
        secret: String,
    },
    PaymentV1 {
        hash: String,
        amount: u64,
        fee: u64,
        nonce: u64,
        payer: String,
        payee: String,
    },
    RewardsV2 {
        hash: String,
        start_epoch: u64,
        end_epoch: u64,
        rewards: Vec<Reward>,
    },
    RewardsV1 {
        hash: String,
        start_epoch: u64,
        end_epoch: u64,
        rewards: Vec<Reward>,
    },
    AssertLocationV2 {
        hash: String,
        fee: u64,
        gain: i64,
        nonce: u64,
        owner: String,
        payer: Option<String>,
        gateway: String,
        location: String,
        elevation: i64,
        staking_fee: u64,
    },
    AssertLocationV1 {
        hash: String,
        fee: u64,
        nonce: u64,
        owner: String,
        payer: Option<String>,
        gateway: String,
        location: String,
        staking_fee: u64,
    },
    AddGatewayV1 {
        hash: String,
        fee: u64,
        owner: String,
        payer: String,
        gateway: String,
        staking_fee: u64,
    },
    TransferHotspotV1 {
        hash: String,
        fee: u64,
        buyer: String,
        seller: String,
        gateway: String,
        buyer_nonce: u64,
        amount_to_seller: u64,
    },
    CreateHtlcV1 {
        fee: u64,
        hash: String,
        nonce: u64,
        payee: String,
        payer: String,
        amount: u64,
        address: String,
        hashlock: String,
        timelock: u64,
    },
    GenGatewayV1 {
        hash: String,
        nonce: u64,
        owner: String,
        gateway: String,
        location: String,
    },
    OuiV1 {
        fee: u64,
        oui: u64,
        hash: String,
        owner: String,
        payer: String,
        filter: String,
        addresses: Vec<String>,
        staking_fee: u64,
        requested_subnet_size: u64,
    },
    RedeemHtlcV1 {
        fee: u64,
        hash: String,
        payee: String,
        address: String,
        preimage: String,
    },
    SecurityCoinbaseV1 {
        hash: String,
        payee: String,
        amount: u64,
    },
    RoutingV1 {
        fee: u64,
        oui: u64,
        hash: String,
        nonce: u64,
        owner: String,
        action: RoutingAction,
    },
    SecurityExchangeV1 {
        fee: u64,
        hash: String,
        nonce: u64,
        payee: String,
        payer: String,
        amount: u64,
    },
    VarsV1 {
        hash: String,
        // skip vars for now
        nonce: u64,
        proof: String,
    },
    TokenBurnV1 {
        fee: u64,
        hash: String,
        memo: String,
        nonce: u64,
        payee: String,
        payer: String,
        amount: u64,
    },
    DcCoinbaseV1 {
        hash: String,
        payee: String,
        amount: u64,
    },
    StateChannelOpenV1 {
        id: String,
        fee: u64,
        oui: u64,
        hash: String,
        nonce: u64,
        owner: String,
        amount: u64,
        expire_within: u64,
    },
    StateChannelCloseV1 {
        hash: String,
        closer: String,
        //this is incomplete
    },
    PriceOracleV1 {
        fee: u64,
        hash: String,
        price: u64,
        public_key: String,
        block_height: u64,
    },
    ValidatorHeartbeatV1 {
        address: String,
        hash: String,
        height: u64,
        signature: String,
        version: u64,
    },
    StakeValidatorV1 {
        address: String,
        fee: u64,
        hash: String,
        owner: String,
        stake: u64,
        owner_signature: String,
    },
    // no examples found on blockchain. inferred from proto source code
    CoinbaseV1 {
        hash: String,
        payee: String,
        amount: u64,
    },
    TokenBurnExchangeRateV1 {
        hash: String,
        rate: u64,
    },
    UpdateGatewayOuiV1 {
        gateway: String,
        hash: String,
        oui: u64,
        nonce: u64,
        fee: u64,
        gateway_owner_signature: String,
        oui_owner_signature: String,
    },
    GenPriceOracleV1 {
        hash: String,
        price: u64,
    },

    #[serde(other)]
    Unknown, // Any other transaction not supported
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use tokio::test;

    #[test]
    async fn txn() {
        let client = Client::default();
        let txn = transactions::get(&client, "1gidN7e6OKn405Fru_0sGhsqca3lTsrfGKrM4dwM_E8")
            .await
            .expect("PocRequestV1");
        match txn {
            Transaction::PocRequestV1 { block_hash, .. } => {
                assert_eq!(block_hash, "RS2mBvd_4pbKCglkkyMroDQekPNO0xDdYx6Te3HGDGg")
            }
            _ => (),
        }
    }
    #[test]
    async fn consensus_group_v1() {
        let client = Client::default();
        let txn = transactions::get(&client, "yh01SJk8dvyqb-BGXxkHFUuLi6wF1pfL0VEFStJUt-E")
            .await
            .expect("ConsensusGroupV1");
        match txn {
            Transaction::ConsensusGroupV1 { hash, .. } => {
                assert_eq!(hash, "yh01SJk8dvyqb-BGXxkHFUuLi6wF1pfL0VEFStJUt-E")
            }
            _ => panic!("Didn't find ConsensusGroupV1"),
        }
    }
    #[test]
    async fn payment_v2() {
        //dosqfzzaYtoGx278w4Xu5dnYt7aSZIkD1-IbtiiLQQM
        let client = Client::default();
        let txn = transactions::get(&client, "C_jJZLKBOv_gRQ6P6wEpZPiRVAjf44FOx1iHOFD4haA")
            .await
            .expect("PaymentV2");
        match txn {
            Transaction::PaymentV2 { payments, .. } => assert_eq!(payments.len(), 1),
            _ => (),
        }
    }
    #[test]
    async fn poc_receipts_v1() {
        let client = Client::default();
        let txn = transactions::get(&client, "8RaF-G4pvMVuIXfBYhdqNuIlFSEHPm_rC8TH-h4JYdE")
            .await
            .expect("PocReceipt");
        match txn {
            Transaction::PocReceiptsV1 { hash, .. } => {
                assert_eq!(hash, "8RaF-G4pvMVuIXfBYhdqNuIlFSEHPm_rC8TH-h4JYdE")
            }
            _ => (),
        }
    }
    #[test]
    async fn payment_v1() {
        let client = Client::default();
        let txn = transactions::get(&client, "iMSckt_hUcMFY_d7W-QOupY0MGq_g3-CC2dq3P-HWIw")
            .await
            .expect("PaymentV1");
        match txn {
            Transaction::PaymentV1 { payee, .. } => {
                assert_eq!(payee, "14YeKFGXE23yAdACj6hu5NWEcYzzKxptYbm5jHgzw9A1P1UQfMv")
            }
            _ => (),
        }
    }
    #[test]
    async fn rewards_v2() {
        let client = Client::default();
        let txn = transactions::get(&client, "X0HNRGZ1HAX51CR8qS6LTopAosjFkuaaKXl850IpNDE")
            .await
            .expect("RewardsV2");
        match txn {
            Transaction::RewardsV2 { rewards, .. } => assert_eq!(rewards.len(), 10138),
            _ => (),
        }
    }
    #[test]
    async fn assert_location_v1() {
        let client = Client::default();
        let txn = transactions::get(&client, "_I16bycHeltuOo7eyqa4uhv2Bc7awcztZflyvRkVZ24")
            .await
            .expect("AssertLocationV1");
        match txn {
            Transaction::AssertLocationV1 { hash, .. } => {
                assert_eq!(hash, "_I16bycHeltuOo7eyqa4uhv2Bc7awcztZflyvRkVZ24")
            }
            _ => (),
        }
    }
    #[test]
    async fn assert_location_v2() {
        let client = Client::default();
        let txn = transactions::get(&client, "TfjRv733Q9FBQ1_unw1c9g5ewVmMBuyf7APuyxKEqrw")
            .await
            .expect("AssertLocationV2");
        match txn {
            Transaction::AssertLocationV2 { gateway, .. } => assert_eq!(
                gateway,
                "112WVxXCrCjiKmmDXLDUJuhYGEHMbXobUZe8oJQkHoMHEFa149a"
            ),
            _ => (),
        }
    }
    #[test]
    async fn add_gateway_v1() {
        let client = Client::default();
        let txn = transactions::get(&client, "aoTggHSgaBAamuUUrXnY42jDZ5WUBxE0k-tshvfn35E")
            .await
            .expect("AddGatewayV1");
        match txn {
            Transaction::AddGatewayV1 { gateway, .. } => assert_eq!(
                gateway,
                "112uuvztDziVQyLVvBxMsovsSPV5ZXkN6uQ5hrWSaWwV1oEZTZtd"
            ),
            _ => (),
        }
    }
    #[test]
    async fn transfer_hotspot_v1() {
        let client = Client::default();
        let txn = transactions::get(&client, "fSFua7A8G41K05QXAvJi5N2OB0QqmQ7xp7u-My4rYHc")
            .await
            .expect("TransferHotspotV1");
        match txn {
            Transaction::TransferHotspotV1 { seller, .. } => assert_eq!(
                seller,
                "14mo9fFGKYFaWh7xscpDLg7misWcuU5xqR8mc8gHr4c43nDnzeX"
            ),
            _ => (),
        }
    }
}
