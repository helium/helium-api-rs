use serde::Deserialize;

use super::BlockStats;

#[derive(Clone, Deserialize, Debug)]
pub struct Stats {
    pub block_times: BlockStats,
    pub challenge_counts: ChallengeCounts,
    pub counts: Counts,
    pub election_times: ElectionTimes,
    pub token_supply: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct TokenSupply {
    pub token_supply: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ChallengeCounts {
    pub active: u64,
    pub last_day: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Counts {
    pub validators: u64,
    pub ouis: u64,
    pub hotspots_dataonly: u64,
    pub blocks: u64,
    pub challenges: u64,
    pub cities: u64,
    pub consensus_groups: u64,
    pub countries: u64,
    pub hotspots: u64,
    pub transactions: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ElectionTimes {
    pub last_day: ElectionTimesMeasures,
    pub last_hour: ElectionTimesMeasures,
    pub last_month: ElectionTimesMeasures,
    pub last_week: ElectionTimesMeasures,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ElectionTimesMeasures {
    pub avg: f64,
    pub stddev: f64,
}
