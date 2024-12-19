use serde::{Deserialize, Deserializer};
use std::str::FromStr;

fn string_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    match s {
        Some(s) if !s.is_empty() => u128::from_str(s).map_err(serde::de::Error::custom),
        _ => Ok(0), // Default value for empty or missing fields
    }
}

fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    match s {
        Some(s) if !s.is_empty() => f64::from_str(s).map_err(serde::de::Error::custom),
        _ => Ok(0.0), // Default value for empty or missing fields
    }
}

// Struct for meta information
#[derive(Deserialize, Debug)]
pub struct RunePoolMeta {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toAssetCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toRuneCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toTradeCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub fromTradeCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthMintCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthRedeemCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub totalCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toAssetVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toRuneVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toTradeVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub fromTradeVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthMintVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthRedeemVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub totalVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toAssetVolumeUSD: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toRuneVolumeUSD: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toTradeVolumeUSD: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub fromTradeVolumeUSD: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthMintVolumeUSD: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthRedeemVolumeUSD: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub totalVolumeUSD: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toAssetFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toRuneFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toTradeFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub fromTradeFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthMintFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthRedeemFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub totalFees: u128,
    #[serde(deserialize_with = "string_to_f64")]
    pub toAssetAverageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub toRuneAverageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub toTradeAverageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub fromTradeAverageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub synthMintAverageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub synthRedeemAverageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub averageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub runePriceUSD: f64,
}

// Struct for interval data
#[derive(Deserialize, Debug)]
pub struct RunePoolInterval {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toAssetCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toRuneCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toTradeCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub fromTradeCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthMintCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthRedeemCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub totalCount: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toAssetVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toRuneVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub toTradeVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub fromTradeVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthMintVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub synthRedeemVolume: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub totalVolume: u128,
    #[serde(deserialize_with = "string_to_f64")]
    pub toAssetAverageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub toRuneAverageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub averageSlip: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub runePriceUSD: f64,
}

// Top-level struct
#[derive(Deserialize, Debug)]
pub struct RootSwapDetails {
    pub meta: RunePoolMeta,
    pub intervals: Vec<RunePoolInterval>,
}
