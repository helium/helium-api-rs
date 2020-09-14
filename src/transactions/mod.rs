use serde::{Serialize, Deserialize};
pub use helium_proto::{
    BlockchainTxnAddGatewayV1 as AddGatewayV1,
    BlockchainTxnAssertLocationV1 as AssertLocationV1,
    BlockchainTxnCoinbaseV1 as CoinbaseV1,
    BlockchainTxnCreateHtlcV1 as CreateHtlcV1,
    BlockchainTxnGenGatewayV1 as GenGatewayV1,
    BlockchainTxnConsensusGroupV1 as ConsensusGroupV1,
    BlockchainTxnOuiV1 as OuiV1,
    BlockchainTxnPaymentV1 as PaymentV1,
    BlockchainTxnPocReceiptsV1 as PocReceiptsV1,
    BlockchainTxnPocRequestV1 as PocRequestV1,
    BlockchainTxnRedeemHtlcV1 as RedeemHtlcV1,
    BlockchainTxnSecurityCoinbaseV1 as SecurityCoinbaseV1,
    BlockchainTxnRoutingV1 as RoutingV1,
    BlockchainTxnSecurityExchangeV1 as SecurityExchangeV1,
    BlockchainTxnVarsV1 as VarsV1,
    BlockchainTxnRewardsV1 as RewardsV1,
    BlockchainTxnTokenBurnV1 as TokenBurnV1,
    BlockchainTxnDcCoinbaseV1 as DcCoinbaseV1,
    BlockchainTxnTokenBurnExchangeRateV1 as TokenBurnExchangeRateV1,
    BlockchainTxnStateChannelOpenV1 as StateChannelOpenV1,
    BlockchainTxnUpdateGatewayOuiV1 as UpdateGatewayOuiV1,
    BlockchainTxnStateChannelCloseV1 as StateChannelCloseV1,
    BlockchainTxnPaymentV2 as PaymentV2,
    BlockchainTxnPriceOracleV1 as PriceOracleV1,
    BlockchainTxnGenPriceOracleV1 as GenPriceOracleV1,
    BlockchainTxnBundleV1 as BundleV1,
};
#[derive(Debug, Serialize, Deserialize)]
struct ApiJson<T>
{
    time: usize,
    height: usize,
    hash: String,
    #[serde(flatten)]
    data: T,
}


#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Transaction {
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

#[cfg(test)]
mod test {
    use serde_json;
    use super::*;

    #[test]
    fn test_poc_request() {
        let data = "{\"version\":2,\"type\":\"poc_request_v1\",\"time\":1599723404,\"secret_hash\":\"dAe3NVp6OLkCbCn3YCYqPgAksupXXOEW5q9WoCJHbig\",\"onion_key_hash\":\"UiOtnd4uWPxTb2gtxijuOAMRPmfK7HFoi3AM6LiIGr4\",\"height\":491103,\"hash\":\"09liXGpHeRmqQ9LxBZSBK3wMN2Npf2UhWGEH4pFxZaI\",\"fee\":0,\"challenger_owner\":\"14oNDCdGp2sCRaiVGeQxdaDtZtTJrs2vcCGiRg1Q9Gxip3rTxRX\",\"challenger_location\":\"8c28d54a8b007ff\",\"challenger\":\"112Uz7DmQ72dbc8cT5Gpm56yqiNuX8w5u99QYEdLaLfxahKzjbdG\",\"block_hash\":\"oPbwJosU4UYAtuXY3r1l3nM6rvCQpn6DR6V7AHzT9U8\"}";
        let parsed: ApiJson<Transaction> = serde_json::from_str(data).unwrap();
        println!("{:?}", parsed)
    }

    #[test]
    fn test_rewards() {
        let data = "{\"type\":\"rewards_v1\",\"time\":1599723944,\"start_epoch\":491083,\"rewards\":[{\"type\":\"poc_challengers\",\"gateway\":\"112rGFR4TPSbrdTAPgP556n9ffst7tE5Xooua9QURbV4XeVoQgiD\",\"amount\":8584986,\"account\":\"13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH\"},{\"type\":\"poc_witnesses\",\"gateway\":\"112rGFR4TPSbrdTAPgP556n9ffst7tE5Xooua9QURbV4XeVoQgiD\",\"amount\":96068025,\"account\":\"13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH\"}],\"height\":491115,\"hash\":\"sadeiZMjqWcJ-sUjjJnZksQRf1SPFkvzoXgJml44UHo\",\"end_epoch\":491114}";
        let parsed: ApiJson<Transaction> = serde_json::from_str(&data).unwrap();
        println!("{:?}", parsed)
    }
}



