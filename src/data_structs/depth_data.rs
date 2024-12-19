use serde::{Deserialize, Deserializer};
use sqlx::PgPool;
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

pub async fn insert_data(pool: &PgPool, data: RootDepthDetails) -> Result<(), sqlx::Error> {
    let meta_start_time: i64 = data.meta.startTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("StartTime {} too large for i64", data.meta.startTime).into())
    })?;

    let meta_end_time: i64 = data.meta.endTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("EndTime {} too large for i64", data.meta.endTime).into())
    })?;

    let meta_price_shift_loss: f64 = data.meta.priceShiftLoss.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!(
                "PriceShiftLoss {} too large for f64",
                data.meta.priceShiftLoss
            )
            .into(),
        )
    })?;

    let meta_luvi_increase: f64 = data.meta.luviIncrease.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!("LuviIncrease {} too large for f64", data.meta.luviIncrease).into(),
        )
    })?;

    let meta_start_asset_depth: i64 = data.meta.startAssetDepth.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!(
                "StartAssetDepth {} too large for i64",
                data.meta.startAssetDepth
            )
            .into(),
        )
    })?;

    let meta_start_rune_depth: i64 = data.meta.startRuneDepth.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!(
                "StartRuneDepth {} too large for i64",
                data.meta.startRuneDepth
            )
            .into(),
        )
    })?;

    let meta_start_lp_units: i64 = data.meta.startLPUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!("StartLPUnits {} too large for i64", data.meta.startLPUnits).into(),
        )
    })?;

    let meta_start_member_count: i64 = data.meta.startMemberCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!(
                "StartMemberCount {} too large for i64",
                data.meta.startMemberCount
            )
            .into(),
        )
    })?;

    let meta_start_synth_units: i64 = data.meta.startSynthUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!(
                "StartSynthUnits {} too large for i64",
                data.meta.startSynthUnits
            )
            .into(),
        )
    })?;

    let meta_end_asset_depth: i64 = data.meta.endAssetDepth.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!(
                "EndAssetDepth {} too large for i64",
                data.meta.endAssetDepth
            )
            .into(),
        )
    })?;

    let meta_end_rune_depth: i64 = data.meta.endRuneDepth.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!("EndRuneDepth {} too large for i64", data.meta.endRuneDepth).into(),
        )
    })?;

    let meta_end_lp_units: i64 = data.meta.endLPUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!("EndLPUnits {} too large for i64", data.meta.endLPUnits).into(),
        )
    })?;

    let meta_end_member_count: i64 = data.meta.endMemberCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!(
                "EndMemberCount {} too large for i64",
                data.meta.endMemberCount
            )
            .into(),
        )
    })?;

    let meta_end_synth_units: i64 = data.meta.endSynthUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!(
                "EndSynthUnits {} too large for i64",
                data.meta.endSynthUnits
            )
            .into(),
        )
    })?;

    sqlx::query(
        r#"
        INSERT INTO Rune_Pool_Depth_Meta (
    startTime, endTime, priceShiftLoss, luviIncrease,
    startAssetDepth, startRuneDepth, startLPUnits, startMemberCount, startSynthUnits,
    endAssetDepth, endRuneDepth, endLPUnits, endMemberCount, endSynthUnits
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,$10, $11, $12, $13, $14)
    "#,
    )
    .bind(meta_start_time)
    .bind(meta_end_time)
    .bind(meta_price_shift_loss)
    .bind(meta_luvi_increase)
    .bind(meta_start_asset_depth)
    .bind(meta_start_rune_depth)
    .bind(meta_start_lp_units)
    .bind(meta_start_member_count)
    .bind(meta_start_synth_units)
    .bind(meta_end_asset_depth)
    .bind(meta_end_rune_depth)
    .bind(meta_end_lp_units)
    .bind(meta_end_member_count)
    .bind(meta_end_synth_units)
    .execute(pool)
    .await?;

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

        let asset_depth: i64 = interval.assetDepth.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!(
                    "Interval AssetDepth {} too large for i64",
                    interval.assetDepth
                )
                .into(),
            )
        })?;

        let rune_depth: i64 = interval.runeDepth.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!(
                    "Interval RuneDepth {} too large for i64",
                    interval.runeDepth
                )
                .into(),
            )
        })?;

        let asset_price: f64 = interval.assetPrice.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!(
                    "Interval AssetPrice {} too large for f64",
                    interval.assetPrice
                )
                .into(),
            )
        })?;

        let asset_price_usd: f64 = interval.assetPriceUSD.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!(
                    "Interval AssetPriceUSD {} too large for f64",
                    interval.assetPriceUSD
                )
                .into(),
            )
        })?;

        let liquidity_units: i64 = interval.liquidityUnits.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!(
                    "Interval LiquidityUnits {} too large for i64",
                    interval.liquidityUnits
                )
                .into(),
            )
        })?;

        let members_count: i64 = interval.membersCount.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!(
                    "Interval MembersCount {} too large for i64",
                    interval.membersCount
                )
                .into(),
            )
        })?;

        let synth_units: i64 = interval.synthUnits.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!(
                    "Interval SynthUnits {} too large for i64",
                    interval.synthUnits
                )
                .into(),
            )
        })?;

        let synth_supply: i64 = interval.synthSupply.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!(
                    "Interval SynthSupply {} too large for i64",
                    interval.synthSupply
                )
                .into(),
            )
        })?;

        let units: i64 = interval.units.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!("Interval Units {} too large for i64", interval.units).into(),
            )
        })?;

        let luvi: f64 = interval.luvi.try_into().map_err(|_| {
            sqlx::Error::Protocol(
                format!("Interval Luvi {} too large for f64", interval.luvi).into(),
            )
        })?;

        sqlx::query(
            r#"
        INSERT INTO Rune_Pool_Depth_Intervals (
    startTime, endTime, assetDepth, runeDepth, assetPrice, assetPriceUSD,
    liquidityUnits, membersCount, synthUnits, synthSupply, units, luvi) 
    VALUES ($1, $2, $3, $4, $5,$6, $7, $8, $9, $10,$11, $12)
    "#,
        )
        .bind(start_time)
        .bind(end_time)
        .bind(asset_depth)
        .bind(rune_depth)
        .bind(asset_price)
        .bind(asset_price_usd)
        .bind(liquidity_units)
        .bind(members_count)
        .bind(synth_units)
        .bind(synth_supply)
        .bind(units)
        .bind(luvi)
        .execute(pool)
        .await?;

        std::println!("Interval inserted successfully!");
    }
    Ok(())
}
