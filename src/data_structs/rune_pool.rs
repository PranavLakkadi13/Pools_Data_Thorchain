use serde::{Deserialize, Deserializer};
use std::str::FromStr;

// Custom function to deserialize a string into a u128
fn string_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    u128::from_str(s).map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Debug)]
pub struct RunePoolIntervalsPriv {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub count: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub units: u128, // Converted to u128
}

#[derive(Deserialize, Debug)]
pub struct RunePoolIntervalsInt {
    pub intervals: Vec<RunePoolIntervalsPriv>,
    pub meta: RunePoolMeta,
}

#[derive(Deserialize, Debug)]
pub struct RunePoolMeta {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub startUnits: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub startCount: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub endUnits: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub endCount: u128, // Converted to u128
}
