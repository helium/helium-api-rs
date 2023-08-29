mod add_gateway_v1;
mod assert_location_v1;
mod assert_location_v2;
mod coinbase_v1;
mod consensus_group_failure_v1;
mod consensus_group_v1;
mod create_htlc_v1;
mod dc_coinbase_v1;
mod gen_gateway_v1;
mod gen_price_oracle_v1;
mod oui_v1;
mod payment_v1;
mod payment_v2;
mod pending_txn_status;
mod poc_receipts_v1;
mod poc_receipts_v2;
mod poc_request_v1;
mod price_oracle_v1;
mod redeem_htlc_v1;
mod reward;
mod rewards_v1;
mod rewards_v2;
mod routing_v1;
mod security_coinbase_v1;
mod security_exchange_v1;
mod stake_validator_v1;
mod state_channel_close_v1;
mod state_channel_open_v1;
mod token_burn_v1;
mod token_exchange_rate_v1;
mod transfer_hotspot_v1;
mod transfer_validator_stake_v1;
mod unstake_validator_v1;
mod update_gateway_oui_v1;
mod validator_heartbeat_v1;
mod vars_v1;

pub use add_gateway_v1::*;
pub use assert_location_v1::*;
pub use assert_location_v2::*;
pub use coinbase_v1::*;
pub use consensus_group_failure_v1::*;
pub use consensus_group_v1::*;
pub use create_htlc_v1::*;
pub use dc_coinbase_v1::*;
pub use gen_gateway_v1::*;
pub use gen_price_oracle_v1::*;
pub use oui_v1::*;
pub use payment_v1::*;
pub use payment_v2::*;
pub use pending_txn_status::*;
pub use poc_receipts_v1::*;
pub use poc_receipts_v2::*;
pub use poc_request_v1::*;
pub use price_oracle_v1::*;
pub use redeem_htlc_v1::*;
pub use reward::*;
pub use rewards_v1::*;
pub use rewards_v2::*;
pub use routing_v1::*;
pub use security_coinbase_v1::*;
pub use security_exchange_v1::*;
pub use stake_validator_v1::*;
pub use state_channel_close_v1::*;
pub use state_channel_open_v1::*;
pub use token_burn_v1::*;
pub use token_exchange_rate_v1::*;
pub use transfer_hotspot_v1::*;
pub use transfer_validator_stake_v1::*;
pub use unstake_validator_v1::*;
pub use update_gateway_oui_v1::*;
pub use validator_heartbeat_v1::*;
pub use vars_v1::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
/// Represents one of various transactions in a block on the chain.
pub enum Transaction {
    AddGatewayV1(AddGatewayV1),
    AssertLocationV1(AssertLocationV1),
    AssertLocationV2(AssertLocationV2),
    CoinbaseV1(CoinbaseV1),
    ConsensusGroupFailureV1(ConsensusGroupFailureV1),
    ConsensusGroupV1(ConsensusGroupV1),
    CreateHtlcV1(CreateHtlcV1),
    DcCoinbaseV1(DcCoinbaseV1),
    GenGatewayV1(GenGatewayV1),
    GenPriceOracleV1(GenPriceOracleV1),
    OuiV1(OuiV1),
    PaymentV1(PaymentV1),
    PaymentV2(PaymentV2),
    PocReceiptsV1(PocReceiptsV1),
    PocReceiptsV2(PocReceiptsV2),
    PocRequestV1(PocRequestV1),
    PriceOracleV1(PriceOracleV1),
    RedeemHtlcV1(RedeemHtlcV1),
    RewardsV1(RewardsV1),
    RewardsV2(RewardsV2),
    RoutingV1(RoutingV1),
    SecurityCoinbaseV1(SecurityCoinbaseV1),
    SecurityExchangeV1(SecurityExchangeV1),
    StakeValidatorV1(StakeValidatorV1),
    StateChannelCloseV1(StateChannelCloseV1),
    StateChannelOpenV1(StateChannelOpenV1),
    TokenBurnExchangeRateV1(TokenBurnExchangeRateV1),
    TokenBurnV1(TokenBurnV1),
    TransferHotspotV1(TransferHotspotV1),
    TransferValidatorStakeV1(TransferValidatorStakeV1),
    UnstakeValidatorV1(UnstakeValidatorV1),
    UpdateGatewayOuiV1(UpdateGatewayOuiV1),
    ValidatorHeartbeatV1(ValidatorHeartbeatV1),
    VarsV1(VarsV1),
    #[serde(other)]
    Unknown, // Any other transaction not supported
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Challenge {
    PocReceiptsV1(PocReceiptsV1),
    PocReceiptsV2(PocReceiptsV2),
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use tokio::test;

    #[test]
    async fn txn() {
        let client = get_test_client();
        let txn = transactions::get(&client, "1gidN7e6OKn405Fru_0sGhsqca3lTsrfGKrM4dwM_E8")
            .await
            .expect("PocRequestV1");
        if let Transaction::PocRequestV1(poc) = txn {
            assert_eq!(
                poc.block_hash,
                "RS2mBvd_4pbKCglkkyMroDQekPNO0xDdYx6Te3HGDGg"
            )
        } else {
            assert!(false)
        }
    }
    #[test]
    async fn consensus_group_v1() {
        let client = get_test_client();
        let txn = transactions::get(&client, "yh01SJk8dvyqb-BGXxkHFUuLi6wF1pfL0VEFStJUt-E")
            .await
            .expect("ConsensusGroupV1");
        if let Transaction::ConsensusGroupV1(cg) = txn {
            assert_eq!(cg.hash, "yh01SJk8dvyqb-BGXxkHFUuLi6wF1pfL0VEFStJUt-E")
        } else {
            assert!(false)
        }
    }
    #[test]
    async fn payment_v2() {
        //dosqfzzaYtoGx278w4Xu5dnYt7aSZIkD1-IbtiiLQQM
        let client = get_test_client();
        let txn = transactions::get(&client, "C_jJZLKBOv_gRQ6P6wEpZPiRVAjf44FOx1iHOFD4haA")
            .await
            .expect("PaymentV2");
        if let Transaction::PaymentV2(p) = txn {
            assert_eq!(p.payments.len(), 1)
        } else {
            assert!(false)
        }
    }
    #[test]
    async fn poc_receipts_v1() {
        let client = get_test_client();
        let txn = transactions::get(&client, "8RaF-G4pvMVuIXfBYhdqNuIlFSEHPm_rC8TH-h4JYdE")
            .await
            .expect("PocReceipt");
        if let Transaction::PocReceiptsV1(pr) = txn {
            assert_eq!(pr.hash, "8RaF-G4pvMVuIXfBYhdqNuIlFSEHPm_rC8TH-h4JYdE")
        } else {
            assert!(false)
        }
    }

    #[test]
    async fn poc_receipts_v2() {
        let client = get_test_client();
        let txn = transactions::get(&client, "077Y_ArUR90ptUMAkiKPp-9NIZfqol5TgGFkoJZa_K8")
            .await
            .expect("PocReceipt");
        if let Transaction::PocReceiptsV2(pr) = txn {
            assert_eq!(pr.hash, "077Y_ArUR90ptUMAkiKPp-9NIZfqol5TgGFkoJZa_K8")
        } else {
            assert!(false)
        }
    }

    #[test]
    async fn payment_v1() {
        let client = get_test_client();
        let txn = transactions::get(&client, "iMSckt_hUcMFY_d7W-QOupY0MGq_g3-CC2dq3P-HWIw")
            .await
            .expect("PaymentV1");
        if let Transaction::PaymentV1(p) = txn {
            assert_eq!(
                p.payee,
                "14YeKFGXE23yAdACj6hu5NWEcYzzKxptYbm5jHgzw9A1P1UQfMv"
            )
        } else {
            assert!(false)
        }
    }
    #[test]
    async fn rewards_v2() {
        let client = get_test_client();
        let txn = transactions::get(&client, "X0HNRGZ1HAX51CR8qS6LTopAosjFkuaaKXl850IpNDE")
            .await
            .expect("RewardsV2");
        if let Transaction::RewardsV2(r) = txn {
            assert_eq!(r.rewards.len(), 10138)
        } else {
            assert!(false)
        }
    }
    #[test]
    async fn assert_location_v1() {
        let client = get_test_client();
        let txn = transactions::get(&client, "_I16bycHeltuOo7eyqa4uhv2Bc7awcztZflyvRkVZ24")
            .await
            .expect("AssertLocationV1");
        if let Transaction::AssertLocationV1(al) = txn {
            assert_eq!(al.hash, "_I16bycHeltuOo7eyqa4uhv2Bc7awcztZflyvRkVZ24")
        } else {
            assert!(false)
        }
    }
    #[test]
    async fn assert_location_v2() {
        let client = get_test_client();
        let txn = transactions::get(&client, "TfjRv733Q9FBQ1_unw1c9g5ewVmMBuyf7APuyxKEqrw")
            .await
            .expect("AssertLocationV2");
        if let Transaction::AssertLocationV2(al) = txn {
            assert_eq!(
                al.gateway,
                "112WVxXCrCjiKmmDXLDUJuhYGEHMbXobUZe8oJQkHoMHEFa149a"
            )
        } else {
            assert!(false)
        }
    }
    #[test]
    async fn add_gateway_v1() {
        let client = get_test_client();
        let txn = transactions::get(&client, "aoTggHSgaBAamuUUrXnY42jDZ5WUBxE0k-tshvfn35E")
            .await
            .expect("AddGatewayV1");
        if let Transaction::AddGatewayV1(ag) = txn {
            assert_eq!(
                ag.gateway,
                "112uuvztDziVQyLVvBxMsovsSPV5ZXkN6uQ5hrWSaWwV1oEZTZtd"
            )
        } else {
            assert!(false)
        }
    }

    #[test]
    async fn add_gateway_v1_error() {
        let client = get_test_client();
        let txn = transactions::get(&client, "ia3c386ZnlVJorvo60WtXEFxy_0w35ImoIdmnW5lpJ8")
            .await
            .expect("AddGatewayV1");
        if let Transaction::AddGatewayV1(ag) = txn {
            assert_eq!(
                ag.gateway,
                "11GPcDmZGniewReZCnpC3SM19Jcw1sQU8W8CUnd7TCh4A6RmEkV"
            )
        } else {
            assert!(false)
        }
    }

    #[test]
    async fn state_channel_close_v1() {
        let client = get_test_client();
        let txn = transactions::get(&client, "vjtEQK0vn1w69fV3TMrlnN6L_qprsoWM_-7DVspmLL8")
            .await
            .expect("StateChannelCloseV1");
        if let Transaction::StateChannelCloseV1(sc) = txn {
            assert_eq!(
                sc.closer,
                "11QVeYckasapcrmqjZqtfGTjE154uHHUvYPPwW6EMwzrpsdr213"
            )
        } else {
            assert!(false)
        }
    }

    #[test]
    async fn transfer_hotspot_v1() {
        let client = get_test_client();
        let txn = transactions::get(&client, "fSFua7A8G41K05QXAvJi5N2OB0QqmQ7xp7u-My4rYHc")
            .await
            .expect("TransferHotspotV1");
        if let Transaction::TransferHotspotV1(th) = txn {
            assert_eq!(
                th.seller,
                "14mo9fFGKYFaWh7xscpDLg7misWcuU5xqR8mc8gHr4c43nDnzeX"
            )
        } else {
            assert!(false)
        }
    }

    #[test]
    async fn transfer_routing_v1_new_xor() {
        let client = get_test_client();

        let txn = transactions::get(&client, "EjL6nBsSxovJluW-kdAaPcEiRt0OPIATOmlHD1Lth4Y")
            .await
            .expect("RoutingV1_NewXor");

        if let Transaction::RoutingV1(r) = txn {
            assert_eq!(r.oui, 12);
            assert_eq!(
                r.owner,
                "112ewJNEUfSg3Jvo276tMjzFC2JzmmZcJJ32CWz2fzYqbyCMMTe1",
            );
            if let RoutingAction::NewXor(nx) = r.action {
                assert_eq!(nx.filter, "wVwCiewtCpELAAAAAAAAAAAAAAAAAAAAf2gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
            } else {
                assert!(false)
            }
        } else {
            assert!(false)
        }
    }

    #[test]
    async fn unstake_validator_v1() {
        let client = get_test_client();

        let txn = transactions::get(&client, "fMT-7_f2WQNKAYIQWX2-V258KsoI61HYbt_zAbN3A1I")
            .await
            .expect("UnstakeValidatorv1");

        if let Transaction::UnstakeValidatorV1(uv) = txn {
            assert_eq!(
                uv.address,
                "11cY9Ly5H3hU4Ai2k7G9niHLAxsKb1ragQYGLJ7E9vh4Vnx6Efb"
            );
            assert_eq!(uv.stake_amount, 1000000000000u64.into());
            assert_eq!(uv.fee, 35000);
        } else {
            assert!(false)
        }
    }

    #[test]
    async fn vars_v1() {
        let client = get_test_client();
        let txn = transactions::get(&client, "SB47bwBKP3ud1KdASYAndxkoIhZCXgPtusLUIsS7Q2o")
            .await
            .expect("VarsV1");
        if let Transaction::VarsV1(v) = txn {
            assert_eq!(
                v.proof,
                "MEUCIAXq0Pi0bK_DutFRF7R7ItEVrdUW2rmY8Guut5bHRboxAiEA9-wrvs7z9QZNRCC7XTKm4sb1cpXFD6TGB8Re8GfOyyA"
            );
            assert!(v.vars.get("sc_max_actors").is_some())
        } else {
            assert!(false)
        }
    }
}
