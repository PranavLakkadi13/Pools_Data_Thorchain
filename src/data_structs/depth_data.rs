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

// Meta struct
#[derive(Deserialize, Debug)]
pub struct RunePoolMeta {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128,
    #[serde(deserialize_with = "string_to_f64")]
    pub priceShiftLoss: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub luviIncrease: f64,
    #[serde(deserialize_with = "string_to_u128")]
    pub startAssetDepth: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub startRuneDepth: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub startLPUnits: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub startMemberCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub startSynthUnits: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endAssetDepth: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endRuneDepth: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endLPUnits: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endMemberCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endSynthUnits: u128,
}

// Interval struct
#[derive(Deserialize, Debug)]
pub struct RunePoolInterval {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub assetDepth: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub runeDepth: u128,
    #[serde(deserialize_with = "string_to_f64")]
    pub assetPrice: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub assetPriceUSD: f64,
    #[serde(deserialize_with = "string_to_u128")]
    pub liquidityUnits: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub membersCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthUnits: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthSupply: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub units: u128,
    #[serde(deserialize_with = "string_to_f64")]
    pub luvi: f64,
}

// Top-level struct
#[derive(Deserialize, Debug)]
pub struct RootDepthDetails {
    pub meta: RunePoolMeta,
    pub intervals: Vec<RunePoolInterval>,
}
