use super::*;
use serde_json;

#[test]
fn test_poc_request() {
    let data = "{\"version\":2,\"type\":\"poc_request_v1\",\"time\":1599723404,\"secret_hash\":\"dAe3NVp6OLkCbCn3YCYqPgAksupXXOEW5q9WoCJHbig\",\"onion_key_hash\":\"UiOtnd4uWPxTb2gtxijuOAMRPmfK7HFoi3AM6LiIGr4\",\"height\":491103,\"hash\":\"09liXGpHeRmqQ9LxBZSBK3wMN2Npf2UhWGEH4pFxZaI\",\"fee\":0,\"challenger_owner\":\"14oNDCdGp2sCRaiVGeQxdaDtZtTJrs2vcCGiRg1Q9Gxip3rTxRX\",\"challenger_location\":\"8c28d54a8b007ff\",\"challenger\":\"112Uz7DmQ72dbc8cT5Gpm56yqiNuX8w5u99QYEdLaLfxahKzjbdG\",\"block_hash\":\"oPbwJosU4UYAtuXY3r1l3nM6rvCQpn6DR6V7AHzT9U8\"}";
    let parsed: Transaction = serde_json::from_str(data).unwrap();

    println!("{:?}", parsed)
}

#[test]
fn test_rewards() {
    let data = "[{\"type\":\"rewards_v1\",\"time\":1599723944,\"start_epoch\":491083,\"rewards\":[{\"type\":\"poc_challengers\",\"gateway\":\"112rGFR4TPSbrdTAPgP556n9ffst7tE5Xooua9QURbV4XeVoQgiD\",\"amount\":8584986,\"account\":\"13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH\"},{\"type\":\"poc_witnesses\",\"gateway\":\"112rGFR4TPSbrdTAPgP556n9ffst7tE5Xooua9QURbV4XeVoQgiD\",\"amount\":96068025,\"account\":\"13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH\"}],\"height\":491115,\"hash\":\"sadeiZMjqWcJ-sUjjJnZksQRf1SPFkvzoXgJml44UHo\",\"end_epoch\":491114}]";
    let parsed: Vec<Transaction> = serde_json::from_str(data).unwrap();
    println!("{:?}", parsed)
}


