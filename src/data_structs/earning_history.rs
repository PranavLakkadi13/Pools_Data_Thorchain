use serde::{Deserialize, Deserializer};
use std::str::FromStr;

// Helper functions for custom deserialization
fn string_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    u128::from_str(s).map_err(serde::de::Error::custom)
}

fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    f64::from_str(s).map_err(serde::de::Error::custom)
}

// Struct for Pool data
#[derive(Deserialize, Debug)]
pub struct PoolData {
    pub pool: String,
    #[serde(deserialize_with = "string_to_u128")]
    pub assetLiquidityFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub runeLiquidityFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub totalLiquidityFeesRune: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub saverEarning: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub rewards: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub earnings: u128,
}

// Struct for Meta data
#[derive(Deserialize, Debug)]
pub struct RunePoolMeta {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub liquidityFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub blockRewards: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub earnings: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub bondingEarnings: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub liquidityEarnings: u128,
    #[serde(deserialize_with = "string_to_f64")]
    pub avgNodeCount: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub runePriceUSD: f64,
    pub pools: Vec<PoolData>,
}

// Struct for Interval data
#[derive(Deserialize, Debug)]
pub struct RunePoolInterval {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub liquidityFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub blockRewards: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub earnings: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub bondingEarnings: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub liquidityEarnings: u128,
    #[serde(deserialize_with = "string_to_f64")]
    pub avgNodeCount: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub runePriceUSD: f64,
    pub pools: Vec<PoolData>,
}

// Top-level struct
#[derive(Deserialize, Debug)]
pub struct RootEarnDetails {
    pub meta: RunePoolMeta,
    pub intervals: Vec<RunePoolInterval>,
}
