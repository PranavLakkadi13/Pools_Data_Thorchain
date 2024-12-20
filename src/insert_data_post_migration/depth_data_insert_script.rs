use crate::data_structs::depth_data::RootDepthDetails;
use sqlx::PgPool;

pub async fn insert_data(pool: &PgPool, data: RootDepthDetails) -> Result<(), sqlx::Error> {
    let meta_start_time: i64 = data.meta.startTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "StartTime {} too large for i64",
            data.meta.startTime
        ))
    })?;

    let meta_end_time: i64 = data.meta.endTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("EndTime {} too large for i64", data.meta.endTime))
    })?;

    let meta_price_shift_loss: f64 = data.meta.priceShiftLoss.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "PriceShiftLoss {} too large for f64",
            data.meta.priceShiftLoss
        ))
    })?;

    let meta_luvi_increase: f64 = data.meta.luviIncrease.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "LuviIncrease {} too large for f64",
            data.meta.luviIncrease
        ))
    })?;

    let meta_start_asset_depth: i64 = data.meta.startAssetDepth.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "StartAssetDepth {} too large for i64",
            data.meta.startAssetDepth
        ))
    })?;

    let meta_start_rune_depth: i64 = data.meta.startRuneDepth.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "StartRuneDepth {} too large for i64",
            data.meta.startRuneDepth
        ))
    })?;

    let meta_start_lp_units: i64 = data.meta.startLPUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "StartLPUnits {} too large for i64",
            data.meta.startLPUnits
        ))
    })?;

    let meta_start_member_count: i64 = data.meta.startMemberCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "StartMemberCount {} too large for i64",
            data.meta.startMemberCount
        ))
    })?;

    let meta_start_synth_units: i64 = data.meta.startSynthUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "StartSynthUnits {} too large for i64",
            data.meta.startSynthUnits
        ))
    })?;

    let meta_end_asset_depth: i64 = data.meta.endAssetDepth.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "EndAssetDepth {} too large for i64",
            data.meta.endAssetDepth
        ))
    })?;

    let meta_end_rune_depth: i64 = data.meta.endRuneDepth.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "EndRuneDepth {} too large for i64",
            data.meta.endRuneDepth
        ))
    })?;

    let meta_end_lp_units: i64 = data.meta.endLPUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "EndLPUnits {} too large for i64",
            data.meta.endLPUnits
        ))
    })?;

    let meta_end_member_count: i64 = data.meta.endMemberCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "EndMemberCount {} too large for i64",
            data.meta.endMemberCount
        ))
    })?;

    let meta_end_synth_units: i64 = data.meta.endSynthUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "EndSynthUnits {} too large for i64",
            data.meta.endSynthUnits
        ))
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
            sqlx::Error::Protocol(format!(
                "Interval StartTime {} too large for i64",
                interval.startTime
            ))
        })?;

        let end_time: i64 = interval.endTime.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval EndTime {} too large for i64",
                interval.endTime
            ))
        })?;

        let asset_depth: i64 = interval.assetDepth.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval AssetDepth {} too large for i64",
                interval.assetDepth
            ))
        })?;

        let rune_depth: i64 = interval.runeDepth.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval RuneDepth {} too large for i64",
                interval.runeDepth
            ))
        })?;

        let asset_price: f64 = interval.assetPrice.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval AssetPrice {} too large for f64",
                interval.assetPrice
            ))
        })?;

        let asset_price_usd: f64 = interval.assetPriceUSD.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval AssetPriceUSD {} too large for f64",
                interval.assetPriceUSD
            ))
        })?;

        let liquidity_units: i64 = interval.liquidityUnits.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval LiquidityUnits {} too large for i64",
                interval.liquidityUnits
            ))
        })?;

        let members_count: i64 = interval.membersCount.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval MembersCount {} too large for i64",
                interval.membersCount
            ))
        })?;

        let synth_units: i64 = interval.synthUnits.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval SynthUnits {} too large for i64",
                interval.synthUnits
            ))
        })?;

        let synth_supply: i64 = interval.synthSupply.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval SynthSupply {} too large for i64",
                interval.synthSupply
            ))
        })?;

        let units: i64 = interval.units.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "Interval Units {} too large for i64",
                interval.units
            ))
        })?;

        let luvi: f64 = interval.luvi.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!("Interval Luvi {} too large for f64", interval.luvi))
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
