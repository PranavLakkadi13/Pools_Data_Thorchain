use crate::data_structs::swap_history::{RunePoolInterval, RunePoolMeta};
use sqlx;

pub async fn insert_rune_pool_meta(
    meta: &RunePoolMeta,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let start_time: i64 = meta.startTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("startTime {} too large for i64", meta.startTime))
    })?;
    let end_time: i64 = meta.endTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("endTime {} too large for i64", meta.endTime))
    })?;
    let to_asset_count: i64 = meta.toAssetCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toAssetCount {} too large for i64",
            meta.toAssetCount
        ))
    })?;
    let to_rune_count: i64 = meta.toRuneCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toRuneCount {} too large for i64",
            meta.toRuneCount
        ))
    })?;
    let to_trade_count: i64 = meta.toTradeCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toTradeCount {} too large for i64",
            meta.toTradeCount
        ))
    })?;
    let from_trade_count: i64 = meta.fromTradeCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "fromTradeCount {} too large for i64",
            meta.fromTradeCount
        ))
    })?;
    let synth_mint_count: i64 = meta.synthMintCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "synthMintCount {} too large for i64",
            meta.synthMintCount
        ))
    })?;
    let synth_redeem_count: i64 = meta.synthRedeemCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "synthRedeemCount {} too large for i64",
            meta.synthRedeemCount
        ))
    })?;
    let total_count: i64 = meta.totalCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("totalCount {} too large for i64", meta.totalCount))
    })?;
    let to_asset_volume: i64 = meta.toAssetVolume.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toAssetVolume {} too large for i64",
            meta.toAssetVolume
        ))
    })?;
    let to_rune_volume: i64 = meta.toRuneVolume.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toRuneVolume {} too large for i64",
            meta.toRuneVolume
        ))
    })?;
    let to_trade_volume: i64 = meta.toTradeVolume.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toTradeVolume {} too large for i64",
            meta.toTradeVolume
        ))
    })?;
    let from_trade_volume: i64 = meta.fromTradeVolume.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "fromTradeVolume {} too large for i64",
            meta.fromTradeVolume
        ))
    })?;
    let synth_mint_volume: i64 = meta.synthMintVolume.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "synthMintVolume {} too large for i64",
            meta.synthMintVolume
        ))
    })?;
    let synth_redeem_volume: i64 = meta.synthRedeemVolume.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "synthRedeemVolume {} too large for i64",
            meta.synthRedeemVolume
        ))
    })?;
    let total_volume: i64 = meta.totalVolume.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "totalVolume {} too large for i64",
            meta.totalVolume
        ))
    })?;
    let to_asset_volume_usd: i64 = meta.toAssetVolumeUSD.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toAssetVolumeUSD {} too large for i64",
            meta.toAssetVolumeUSD
        ))
    })?;
    let to_rune_volume_usd: i64 = meta.toRuneVolumeUSD.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toRuneVolumeUSD {} too large for i64",
            meta.toRuneVolumeUSD
        ))
    })?;
    let to_trade_volume_usd: i64 = meta.toTradeVolumeUSD.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toTradeVolumeUSD {} too large for i64",
            meta.toTradeVolumeUSD
        ))
    })?;
    let from_trade_volume_usd: i64 = meta.fromTradeVolumeUSD.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "fromTradeVolumeUSD {} too large for i64",
            meta.fromTradeVolumeUSD
        ))
    })?;
    let synth_mint_volume_usd: i64 = meta.synthMintVolumeUSD.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "synthMintVolumeUSD {} too large for i64",
            meta.synthMintVolumeUSD
        ))
    })?;
    let synth_redeem_volume_usd: i64 = meta.synthRedeemVolumeUSD.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "synthRedeemVolumeUSD {} too large for i64",
            meta.synthRedeemVolumeUSD
        ))
    })?;
    let total_volume_usd: i64 = meta.totalVolumeUSD.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "totalVolumeUSD {} too large for i64",
            meta.totalVolumeUSD
        ))
    })?;
    let to_asset_fees: i64 = meta.toAssetFees.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toAssetFees {} too large for i64",
            meta.toAssetFees
        ))
    })?;
    let to_rune_fees: i64 = meta.toRuneFees.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("toRuneFees {} too large for i64", meta.toRuneFees))
    })?;
    let to_trade_fees: i64 = meta.toTradeFees.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "toTradeFees {} too large for i64",
            meta.toTradeFees
        ))
    })?;
    let from_trade_fees: i64 = meta.fromTradeFees.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "fromTradeFees {} too large for i64",
            meta.fromTradeFees
        ))
    })?;
    let synth_mint_fees: i64 = meta.synthMintFees.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "synthMintFees {} too large for i64",
            meta.synthMintFees
        ))
    })?;
    let synth_redeem_fees: i64 = meta.synthRedeemFees.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "synthRedeemFees {} too large for i64",
            meta.synthRedeemFees
        ))
    })?;
    let total_fees: i64 = meta.totalFees.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("totalFees {} too large for i64", meta.totalFees))
    })?;
    let to_asset_average_slip: f64 = meta.toAssetAverageSlip;
    let to_rune_average_slip: f64 = meta.toRuneAverageSlip;
    let to_trade_average_slip: f64 = meta.toTradeAverageSlip;
    let from_trade_average_slip: f64 = meta.fromTradeAverageSlip;
    let synth_mint_average_slip: f64 = meta.synthMintAverageSlip;
    let synth_redeem_average_slip: f64 = meta.synthRedeemAverageSlip;
    let average_slip: f64 = meta.averageSlip;
    let rune_price_usd: f64 = meta.runePriceUSD;

    // Bind each converted field in the same order as the placeholders in the SQL query
    sqlx::query(
        r#"
        INSERT INTO swap_data_rune_pool_meta (
            start_time, end_time, to_asset_count, to_rune_count, to_trade_count, from_trade_count,
            synth_mint_count, synth_redeem_count, total_count, to_asset_volume, to_rune_volume,
            to_trade_volume, from_trade_volume, synth_mint_volume, synth_redeem_volume, total_volume,
            to_asset_volume_usd, to_rune_volume_usd, to_trade_volume_usd, from_trade_volume_usd,
            synth_mint_volume_usd, synth_redeem_volume_usd, total_volume_usd, to_asset_fees, to_rune_fees,
            to_trade_fees, from_trade_fees, synth_mint_fees, synth_redeem_fees, total_fees,
            to_asset_average_slip, to_rune_average_slip, to_trade_average_slip, from_trade_average_slip,
            synth_mint_average_slip, synth_redeem_average_slip, average_slip, rune_price_usd
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,
            $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
            $31, $32, $33, $34, $35, $36, $37, $38
        ) RETURNING id
        "#,
    )
    .bind(start_time)
    .bind(end_time)
    .bind(to_asset_count)
    .bind(to_rune_count)
    .bind(to_trade_count)
    .bind(from_trade_count)
    .bind(synth_mint_count)
    .bind(synth_redeem_count)
    .bind(total_count)
    .bind(to_asset_volume)
    .bind(to_rune_volume)
    .bind(to_trade_volume)
    .bind(from_trade_volume)
    .bind(synth_mint_volume)
    .bind(synth_redeem_volume)
    .bind(total_volume)
    .bind(to_asset_volume_usd)
    .bind(to_rune_volume_usd)
    .bind(to_trade_volume_usd)
    .bind(from_trade_volume_usd)
    .bind(synth_mint_volume_usd)
    .bind(synth_redeem_volume_usd)
    .bind(total_volume_usd)
    .bind(to_asset_fees)
    .bind(to_rune_fees)
    .bind(to_trade_fees)
    .bind(from_trade_fees)
    .bind(synth_mint_fees)
    .bind(synth_redeem_fees)
    .bind(total_fees)
    .bind(to_asset_average_slip)
    .bind(to_rune_average_slip)
    .bind(to_trade_average_slip)
    .bind(from_trade_average_slip)
    .bind(synth_mint_average_slip)
    .bind(synth_redeem_average_slip)
    .bind(average_slip)
    .bind(rune_price_usd)
    .fetch_one(pool)
    .await?;

    std::println!("The data has been inserted into swap_data_rune_pool_meta");

    Ok(())
}

pub async fn insert_rune_pool_intervals(
    intervals: &[RunePoolInterval], // Accepts a slice of intervals
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    for interval in intervals {
        let start_time: i64 = interval.startTime.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "startTime {} too large for i64",
                interval.startTime
            ))
        })?;
        let end_time: i64 = interval.endTime.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!("endTime {} too large for i64", interval.endTime))
        })?;
        let to_asset_count: i64 = interval.toAssetCount.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "toAssetCount {} too large for i64",
                interval.toAssetCount
            ))
        })?;
        let to_rune_count: i64 = interval.toRuneCount.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "toRuneCount {} too large for i64",
                interval.toRuneCount
            ))
        })?;
        let to_trade_count: i64 = interval.toTradeCount.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "toTradeCount {} too large for i64",
                interval.toTradeCount
            ))
        })?;
        let from_trade_count: i64 = interval.fromTradeCount.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "fromTradeCount {} too large for i64",
                interval.fromTradeCount
            ))
        })?;
        let synth_mint_count: i64 = interval.synthMintCount.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "synthMintCount {} too large for i64",
                interval.synthMintCount
            ))
        })?;
        let synth_redeem_count: i64 = interval.synthRedeemCount.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "synthRedeemCount {} too large for i64",
                interval.synthRedeemCount
            ))
        })?;
        let total_count: i64 = interval.totalCount.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "totalCount {} too large for i64",
                interval.totalCount
            ))
        })?;
        let to_asset_volume: i64 = interval.toAssetVolume.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "toAssetVolume {} too large for i64",
                interval.toAssetVolume
            ))
        })?;
        let to_rune_volume: i64 = interval.toRuneVolume.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "toRuneVolume {} too large for i64",
                interval.toRuneVolume
            ))
        })?;
        let to_trade_volume: i64 = interval.toTradeVolume.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "toTradeVolume {} too large for i64",
                interval.toTradeVolume
            ))
        })?;
        let from_trade_volume: i64 = interval.fromTradeVolume.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "fromTradeVolume {} too large for i64",
                interval.fromTradeVolume
            ))
        })?;
        let synth_mint_volume: i64 = interval.synthMintVolume.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "synthMintVolume {} too large for i64",
                interval.synthMintVolume
            ))
        })?;
        let synth_redeem_volume: i64 = interval.synthRedeemVolume.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "synthRedeemVolume {} too large for i64",
                interval.synthRedeemVolume
            ))
        })?;
        let total_volume: i64 = interval.totalVolume.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "totalVolume {} too large for i64",
                interval.totalVolume
            ))
        })?;
        let rune_price_usd: f64 = interval.runePriceUSD;

        // Insert each interval data into the database
        sqlx::query(
            r#"
        INSERT INTO swap_data_rune_pool_interval (
            start_time, end_time, to_asset_count, to_rune_count, to_trade_count, from_trade_count,
            synth_mint_count, synth_redeem_count, total_count, to_asset_volume, to_rune_volume,
            to_trade_volume, from_trade_volume, synth_mint_volume, synth_redeem_volume, total_volume,
            to_asset_average_slip, to_rune_average_slip, average_slip, rune_price_usd
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,
            $17, $18, $19, $20
        )
        "#,
        )
        .bind(start_time)
        .bind(end_time)
        .bind(to_asset_count)
        .bind(to_rune_count)
        .bind(to_trade_count)
        .bind(from_trade_count)
        .bind(synth_mint_count)
        .bind(synth_redeem_count)
        .bind(total_count)
        .bind(to_asset_volume)
        .bind(to_rune_volume)
        .bind(to_trade_volume)
        .bind(from_trade_volume)
        .bind(synth_mint_volume)
        .bind(synth_redeem_volume)
        .bind(total_volume)
        .bind(interval.toAssetAverageSlip)
        .bind(interval.toRuneAverageSlip)
        .bind(interval.averageSlip)
        .bind(rune_price_usd)
        .execute(pool)
        .await?;

        std::println!("Interval data inserted successfully.");
    }

    Ok(())
}
