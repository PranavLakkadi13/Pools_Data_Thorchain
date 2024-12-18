use serde::{Deserialize, Deserializer};
use sqlx::postgres::PgPool;
use sqlx::prelude::*;
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

async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS SwapDetailsRunePoolMeta (
            id SERIAL PRIMARY KEY,
            startTime BIGINT,
            endTime BIGINT,
            to_asset_count BIGINT,
            to_rune_count BIGINT,
            to_trade_count BIGINT,
            from_trade_count BIGINT,
            synth_mint_count BIGINT,
            synth_redeem_count BIGINT,
            total_count BIGINT,
            to_asset_volume BIGINT,
            to_rune_volume BIGINT,
            to_trade_volume BIGINT,
            from_trade_volume BIGINT,
            synth_mint_volume BIGINT,
            synth_redeem_volume BIGINT,
            total_volume BIGINT,
            to_asset_volume_usd BIGINT,
            to_rune_volume_usd BIGINT,
            to_trade_volume_usd BIGINT,
            from_trade_volume_usd BIGINT,
            synth_mint_volume_usd BIGINT,
            synth_redeem_volume_usd BIGINT,
            total_volume_usd BIGINT,
            to_asset_fees BIGINT,
            to_rune_fees BIGINT,
            to_trade_fees BIGINT,
            from_trade_fees BIGINT,
            synth_mint_fees BIGINT,
            synth_redeem_fees BIGINT,
            total_fees BIGINT,
            to_asset_average_slip BIGINT,
            to_rune_average_slip BIGINT,
            to_trade_average_slip BIGINT,
            from_trade_average_slip BIGINT,
            synth_mint_average_slip BIGINT,
            synth_redeem_average_slip BIGINT,
            average_slip BIGINT,
            rune_price_usd BIGINT
        );
         "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS SwapDetailsRunePoolIntervals (
            id SERIAL PRIMARY KEY,
            startTime BIGINT,
            endTime BIGINT,
            to_asset_count BIGINT,
            to_rune_count BIGINT,
            to_trade_count BIGINT,
            from_trade_count BIGINT,
            synth_mint_count BIGINT,
            synth_redeem_count BIGINT,
            total_count BIGINT,
            to_asset_volume BIGINT,
            to_rune_volume BIGINT,
            to_trade_volume BIGINT,
            from_trade_volume BIGINT,
            synth_mint_volume BIGINT,
            synth_redeem_volume BIGINT,
            total_volume BIGINT,
            to_asset_average_slip BIGINT,
            to_rune_average_slip BIGINT,
            average_slip BIGINT,
            rune_price_usd BIGINT
        )
    "#,
    )
    .execute(pool)
    .await?;

    println!("Table created successfully!");

    Ok(())
}

async fn insert_data(pool: &PgPool, interval: RunePoolInterval) -> Result<(), sqlx::Error> {
    for interval in &data.intervals {
        let start_time: i64 = interval.startTime.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!(
                    "Interval StartTime {} too large for i64",
                    interval.startTime
                )
                .into(),
            )
        })?;
        let end_time: i64 = interval.endTime.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!("Interval EndTime {} too large for i64", interval.endTime).into(),
            )
        })?;
        let count: i64 = interval.count.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!("Interval Count {} too large for i64", interval.count).into(),
            )
        })?;
        let units: i64 = interval.units.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!("Interval Units {} too large for i64", interval.units).into(),
            )
        })?;

        sqlx::query(
        r#"
        INSERT INTO RunePoolIntervals (
            startTime, endTime, to_asset_count, to_rune_count, to_trade_count,
            from_trade_count, synth_mint_count, synth_redeem_count, total_count,
            to_asset_volume, to_rune_volume, to_trade_volume, from_trade_volume,
            synth_mint_volume, synth_redeem_volume, total_volume, to_asset_average_slip,
            to_rune_average_slip, average_slip, rune_price_usd
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20
        )
    "#
    )
    .bind(&interval.startTime)
    .bind(&interval.endTime)
    .bind(&interval.to_asset_count)
    .bind(&interval.to_rune_count)
    .bind(&interval.to_trade_count)
    .bind(&interval.from_trade_count)
    .bind(&interval.synth_mint_count)
    .bind(&interval.synth_redeem_count)
    .bind(&interval.total_count)
    .bind(&interval.to_asset_volume)
    .bind(&interval.to_rune_volume)
    .bind(&interval.to_trade_volume)
    .bind(&interval.from_trade_volume)
    .bind(&interval.synth_mint_volume)
    .bind(&interval.synth_redeem_volume)
    .bind(&interval.total_volume)
    .bind(&interval.to_asset_average_slip)
    .bind(&interval.to_rune_average_slip)
    .bind(&interval.average_slip)
    .bind(&interval.rune_price_usd)
    .execute(pool).await?;
    }
    println!("Data inserted successfully!");

    Ok(())
}
