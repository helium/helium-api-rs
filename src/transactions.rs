pub use helium_proto::*;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub time: usize,
    #[serde(default)]
    pub height: Option<usize>,
    pub hash: String,
    #[serde(flatten)]
    pub data: Data,
}

// by default, Api's version of the txn has at least the proto data
// custom structs may be added with more "value-added" fields
macro_rules! default_data {
    ($txn:tt, $txn_proto:tt) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $txn {
            #[serde(flatten)]
            pub proto: $txn_proto,
        }
    };
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Data {
    AddGatewayV1(AddGatewayV1),
    AssertLocationV1(AssertLocationV1),
    CoinbaseV1(CoinbaseV1),
    CreateHtlcV1(CreateHtlcV1),
    GenGatewayV1(GenGatewayV1),
    ConsensusGroupV1(ConsensusGroupV1),
    OuiV1(OuiV1),
    PaymentV1(PaymentV1),
    PocReceiptsV1(PocReceiptsV1),
    PocRequestV1(PocRequestV1),
    RedeemHtlcV1(RedeemHtlcV1),
    SecurityCoinbaseV1(SecurityCoinbaseV1),
    RoutingV1(RoutingV1),
    SecurityExchangeV1(SecurityExchangeV1),
    VarsV1(VarsV1),
    RewardsV1(RewardsV1),
    TokenBurnV1(TokenBurnV1),
    DcCoinbaseV1(DcCoinbaseV1),
    TokenBurnExchangeRateV1(TokenBurnExchangeRateV1),
    StateChannelOpenV1(StateChannelOpenV1),
    UpdateGatewayOuiV1(UpdateGatewayOuiV1),
    StateChannelCloseV1(Box<StateChannelCloseV1>),
    PaymentV2(PaymentV2),
    PriceOracleV1(PriceOracleV1),
    GenPriceOracleV1(GenPriceOracleV1),
    BundleV1(BundleV1),
}

default_data!(AddGatewayV1, BlockchainTxnAddGatewayV1);
default_data!(AssertLocationV1, BlockchainTxnAssertLocationV1);
default_data!(CoinbaseV1, BlockchainTxnCoinbaseV1);
default_data!(CreateHtlcV1, BlockchainTxnCreateHtlcV1);
default_data!(GenGatewayV1, BlockchainTxnGenGatewayV1);
default_data!(ConsensusGroupV1, BlockchainTxnConsensusGroupV1);
default_data!(OuiV1, BlockchainTxnOuiV1);
default_data!(PaymentV1, BlockchainTxnPaymentV1);
default_data!(PocReceiptsV1, BlockchainTxnPocReceiptsV1);
default_data!(PocRequestV1, BlockchainTxnPocRequestV1);
default_data!(RedeemHtlcV1, BlockchainTxnRedeemHtlcV1);
default_data!(SecurityCoinbaseV1, BlockchainTxnSecurityCoinbaseV1);
default_data!(RoutingV1, BlockchainTxnRoutingV1);
default_data!(SecurityExchangeV1, BlockchainTxnSecurityExchangeV1);
default_data!(VarsV1, BlockchainTxnVarsV1);
default_data!(RewardsV1, BlockchainTxnRewardsV1);
default_data!(TokenBurnV1, BlockchainTxnTokenBurnV1);
default_data!(DcCoinbaseV1, BlockchainTxnDcCoinbaseV1);
default_data!(
    TokenBurnExchangeRateV1,
    BlockchainTxnTokenBurnExchangeRateV1
);
default_data!(StateChannelOpenV1, BlockchainTxnStateChannelOpenV1);
default_data!(UpdateGatewayOuiV1, BlockchainTxnUpdateGatewayOuiV1);
default_data!(StateChannelCloseV1, BlockchainTxnStateChannelCloseV1);
default_data!(PaymentV2, BlockchainTxnPaymentV2);
default_data!(PriceOracleV1, BlockchainTxnPriceOracleV1);
default_data!(GenPriceOracleV1, BlockchainTxnGenPriceOracleV1);
default_data!(BundleV1, BlockchainTxnBundleV1);

// Use the JSON Serializiation for debug printing
use std::fmt;
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = serde_json::to_string(self).unwrap();

        write!(f, "{}", str)
    }
}
impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = serde_json::to_string(self).unwrap();

        f.write_str(&str)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn test_poc_request() {
        let data = "{\"version\":2,\"type\":\"poc_request_v1\",\"time\":1599723404,\"secret_hash\":\"dAe3NVp6OLkCbCn3YCYqPgAksupXXOEW5q9WoCJHbig\",\"onion_key_hash\":\"UiOtnd4uWPxTb2gtxijuOAMRPmfK7HFoi3AM6LiIGr4\",\"height\":491103,\"hash\":\"09liXGpHeRmqQ9LxBZSBK3wMN2Npf2UhWGEH4pFxZaI\",\"fee\":0,\"challenger_owner\":\"14oNDCdGp2sCRaiVGeQxdaDtZtTJrs2vcCGiRg1Q9Gxip3rTxRX\",\"challenger_location\":\"8c28d54a8b007ff\",\"challenger\":\"112Uz7DmQ72dbc8cT5Gpm56yqiNuX8w5u99QYEdLaLfxahKzjbdG\",\"block_hash\":\"oPbwJosU4UYAtuXY3r1l3nM6rvCQpn6DR6V7AHzT9U8\"}";
        let parsed: Transaction<Data> = serde_json::from_str(data).unwrap();
        println!("{:?}", parsed)
    }

    #[test]
    fn test_rewards() {
        let data = "{\"type\":\"rewards_v1\",\"time\":1599723944,\"start_epoch\":491083,\"rewards\":[{\"type\":\"poc_challengers\",\"gateway\":\"112rGFR4TPSbrdTAPgP556n9ffst7tE5Xooua9QURbV4XeVoQgiD\",\"amount\":8584986,\"account\":\"13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH\"},{\"type\":\"poc_witnesses\",\"gateway\":\"112rGFR4TPSbrdTAPgP556n9ffst7tE5Xooua9QURbV4XeVoQgiD\",\"amount\":96068025,\"account\":\"13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH\"}],\"height\":491115,\"hash\":\"sadeiZMjqWcJ-sUjjJnZksQRf1SPFkvzoXgJml44UHo\",\"end_epoch\":491114}";
        let parsed: Transaction<Data> = serde_json::from_str(&data).unwrap();
        println!("{:?}", parsed)
    }
}
