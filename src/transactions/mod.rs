pub(crate) mod custom_serde;
mod types;
pub use types::*;

#[cfg(test)]
mod tests;

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
    RoutingV1(TxnRoutingV1),
    SecurityExchangeV1(SecurityExchangeV1),
    VarsV1(VarsV1),
    RewardsV1(RewardV1),
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CoinbaseV1 {
    pub payee: Pubkey,
    pub amount: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SecurityCoinbaseV1 {
    pub payee: Pubkey,
    pub amount: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OuiV1 {
    pub owner: Pubkey,
    pub addresses: Vec<Pubkey>,
    pub filter: DataField,
    pub requested_subnet_size: u32,
    pub payer: Pubkey,
    pub staking_fee: u64,
    pub fee: u64,
    pub owner_signature: Option<Signature>,
    pub payer_signature: Option<Signature>,
    pub oui: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GenGatewayV1 {
    pub gateway: Pubkey,
    pub owner: Pubkey,
    pub location: DataField,
    pub nonce: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateRouters {
    pub router_addresses: ::std::vec::Vec<Pubkey>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateXor {
    pub index: u32,
    pub filter: DataField,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TxnRoutingV1 {
    pub oui: u32,
    pub owner: Pubkey,
    pub fee: u64,
    pub nonce: u64,
    pub signature: Option<Signature>,
    pub staking_fee: Option<u64>,
    pub update: ::std::option::Option<blockchain_txn_routing_v1::Update>,
}

pub mod blockchain_txn_routing_v1 {
    use super::DataField;
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub enum Update {
        UpdateRouters(super::UpdateRouters),
        NewXor(DataField),
        UpdateXor(super::UpdateXor),
        RequestSubnet(u32),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PaymentV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub signature: Option<Signature>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SecurityExchangeV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub signature: Option<Signature>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConsensusGroupV1 {
    pub members: ::std::vec::Vec<Pubkey>,
    pub proof: DataField,
    pub height: u64,
    pub delay: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddGatewayV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub owner: Pubkey,
    pub gateway: Pubkey,
    pub owner_signature: Option<Signature>,
    pub gateway_signature: Option<Signature>,
    pub payer: Pubkey,
    pub payer_signature: Signature,
    pub staking_fee: u64,
    pub fee: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AssertLocationV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub gateway: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub gateway_signature: Option<Signature>,
    pub owner_signature: Option<Signature>,
    pub payer_signature: Option<Signature>,
    pub location: String,
    pub nonce: u64,
    pub staking_fee: u64,
    pub fee: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateHtlcV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub address: DataField,
    pub hashlock: DataField,
    pub timelock: u64,
    pub amount: u64,
    pub fee: u64,
    pub signature: Option<Signature>,
    pub nonce: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RedeemHtlcV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub payee: Pubkey,
    pub address: DataField,
    pub preimage: DataField,
    pub fee: u64,
    pub signature: Option<Signature>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PocRequestV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub challenger: Pubkey,
    pub secret_hash: Hash,
    pub onion_key_hash: Hash,
    pub block_hash: Hash,
    pub fee: u64,
    pub signature: Option<Signature>,
    pub version: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockchainPocReceiptV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub gateway: Pubkey,
    pub timestamp: u64,
    pub signal: i32,
    pub data: DataField,
    pub origin: Origin,
    pub signature: Option<Signature>,
    pub snr: f32,
    pub frequency: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockchainPocWitnessV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub gateway: Pubkey,
    pub timestamp: u64,
    pub signal: i32,
    pub packet_hash: DataField,
    pub signature: Option<Signature>,
    pub snr: f32,
    pub frequency: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockchainPocResponseV1 {
    pub payload: ::std::option::Option<blockchain_poc_response_v1::Payload>,
}

pub mod blockchain_poc_response_v1 {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub enum Payload {
        Receipt(super::BlockchainPocReceiptV1),
        Witness(super::BlockchainPocWitnessV1),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockchainPocPathElementV1 {
    pub challengee: Pubkey,
    pub receipt: ::std::option::Option<BlockchainPocReceiptV1>,
    pub witnesses: ::std::vec::Vec<BlockchainPocWitnessV1>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PocReceiptsV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub challenger: Pubkey,
    pub secret: DataField,
    pub onion_key_hash: Hash,
    pub path: ::std::vec::Vec<BlockchainPocPathElementV1>,
    pub fee: u64,
    pub signature: Option<Signature>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    P2p,
    Radio,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockchainVarV1 {
    pub name: std::string::String,
    pub r#type: std::string::String,
    pub value: DataField,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VarsV1 {
    pub vars: ::std::vec::Vec<BlockchainVarV1>,
    pub version_predicate: u32,
    pub proof: DataField,
    pub master_key: DataField,
    pub key_proof: DataField,
    pub cancels: ::std::vec::Vec<DataField>,
    pub unsets: ::std::vec::Vec<DataField>,
    pub nonce: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Reward {
    Securities(RewardData),
    DataCredits(RewardData),
    PocChallengees(RewardData),
    PocChallengers(RewardData),
    PocWitnesses(RewardData),
    Consensus(RewardData),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RewardData {
    pub account: Pubkey,
    pub gateway: Pubkey,
    pub amount: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RewardV1 {
    pub time: usize,
    pub height: u64,
    pub hash: Hash,
    pub start_epoch: usize,
    pub rewards: Vec<Reward>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenBurnV1 {
    pub time: usize,
    pub height: u64,
    pub hash: Hash,
    pub payer: Pubkey,
    pub payee: Pubkey,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Option<Signature>,
    pub fee: Option<u64>,
    // pub memo: Memo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DcCoinbaseV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub payee: Pubkey,
    pub amount: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenBurnExchangeRateV1 {
    pub rate: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StateChannelOpenV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub id: DataField,
    pub owner: Pubkey,
    pub amount: i64,
    pub expire_within: i64,
    pub oui: u64,
    pub nonce: u64,
    pub signature: Option<Signature>,
    pub fee: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateGatewayOuiV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub gateway: Pubkey,
    pub oui: u64,
    pub nonce: u64,
    pub fee: u64,
    pub gateway_owner_signature: Option<Signature>,
    pub oui_owner_signature: Option<Signature>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Eui {
    pub deveui: u64,
    pub appeui: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RoutingInformation {
    pub data: ::std::option::Option<routing_information::Data>,
}

pub mod routing_information {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub enum Data {
        Devaddr(u32),
        Eui(super::Eui),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StateChannelSummary {
    pub client_pubkeybin: Option<Signature>,
    pub num_packets: u64,
    pub num_dcs: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StateChannelV1 {
    pub id: DataField,
    pub owner: Pubkey,
    //    pub credits: u64,
    pub nonce: u64,
    pub summaries: ::std::vec::Vec<StateChannelSummary>,
    pub root_hash: String,
    //    pub skewed: DataField,
    pub state: state_channel_v1::State,
    pub expire_at_block: u64,
    pub signature: Option<Signature>,
}

pub mod state_channel_v1 {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "snake_case")]
    pub enum State {
        Open,
        Closed,
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StateChannelCloseV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub state_channel: ::std::option::Option<StateChannelV1>,
    pub closer: Pubkey,
    pub signature: Option<Signature>,
    pub fee: Option<u64>,
    pub conflicts_with: ::std::option::Option<StateChannelV1>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PaymentV2 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub payer: Pubkey,
    pub payments: ::std::vec::Vec<Payment>,
    pub fee: u64,
    pub nonce: u64,
    pub signature: Option<Signature>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Payment {
    pub payee: Pubkey,
    pub amount: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PriceOracleV1 {
    pub time: usize,
    pub height: u64,
    pub hash: String,
    pub public_key: Pubkey,
    pub price: u64,
    pub block_height: u64,
    pub signature: Option<Signature>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GenPriceOracleV1 {
    pub price: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BundleV1 {
    pub transactions: ::std::vec::Vec<BlockchainTxn>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockchainTxn {
    pub txn: ::std::option::Option<Transaction>,
}
