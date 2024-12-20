use crate::data_structs::earning_history::{PoolData, RunePoolInterval, RunePoolMeta};
use sqlx;

pub async fn insert_pool_data(
    pool_data: &PoolData,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    // Helper function to convert f64 to i64 with range checking
    fn f64_to_i64(value: f64, field_name: &str) -> Result<i64, sqlx::Error> {
        if value.is_finite() && value >= i64::MIN as f64 && value <= i64::MAX as f64 {
            Ok(value.round() as i64)
        } else {
            Err(sqlx::Error::Protocol(format!(
                "{} value {} is out of range for i64",
                field_name, value
            )))
        }
    }

    // Use pool string directly
    let pool_name = &pool_data.pool;

    // Convert each field with the helper function
    let asset_liquidity_fees = f64_to_i64(pool_data.assetLiquidityFees, "assetLiquidityFees")?;
    let rune_liquidity_fees = f64_to_i64(pool_data.runeLiquidityFees, "runeLiquidityFees")?;
    let total_liquidity_fees_rune =
        f64_to_i64(pool_data.totalLiquidityFeesRune, "totalLiquidityFeesRune")?;
    let saver_earning = f64_to_i64(pool_data.saverEarning, "saverEarning")?;
    let rewards = f64_to_i64(pool_data.rewards, "rewards")?;
    let earnings = f64_to_i64(pool_data.earnings, "earnings")?;

    // Insert into the database
    sqlx::query(
        r#"
        INSERT INTO Earning_Data_Pool_Data (
            pool, asset_liquidity_fees, rune_liquidity_fees, total_liquidity_fees_rune, 
            saver_earning, rewards, earnings
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(pool_name)
    .bind(asset_liquidity_fees)
    .bind(rune_liquidity_fees)
    .bind(total_liquidity_fees_rune)
    .bind(saver_earning)
    .bind(rewards)
    .bind(earnings)
    .fetch_one(pool)
    .await?;

    // Log insertion success
    std::println!("The data has been inserted into earning_data_pool_data");

    Ok(())
}

pub async fn insert_rune_pool_meta(
    meta: &RunePoolMeta,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    // Convert meta.startTime to i64 with error handling
    let meta_start_time: i64 = meta.startTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("startTime {} too large for i64", meta.startTime))
    })?;

    // Convert meta.endTime to i64 with error handling
    let meta_end_time: i64 = meta.endTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("endTime {} too large for i64", meta.endTime))
    })?;

    // Convert meta.liquidityFees to i64 with error handling
    let meta_liquidity_fees: i64 = meta.liquidityFees.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "liquidityFees {} too large for i64",
            meta.liquidityFees
        ))
    })?;

    // Convert meta.blockRewards to i64 with error handling
    let meta_block_rewards: i64 = meta.blockRewards.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "blockRewards {} too large for i64",
            meta.blockRewards
        ))
    })?;

    // Convert meta.earnings to i64 with error handling
    let meta_earnings: i64 = meta.earnings.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("earnings {} too large for i64", meta.earnings))
    })?;

    // Convert meta.bondingEarnings to i64 with error handling
    let meta_bonding_earnings: i64 = meta.bondingEarnings.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "bondingEarnings {} too large for i64",
            meta.bondingEarnings
        ))
    })?;

    // Convert meta.liquidityEarnings to i64 with error handling
    let meta_liquidity_earnings: i64 = meta.liquidityEarnings.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!(
            "liquidityEarnings {} too large for i64",
            meta.liquidityEarnings
        ))
    })?;

    // Use f64 directly for avgNodeCount and runePriceUSD as they do not need conversion
    let avg_node_count = meta.avgNodeCount;
    let rune_price_usd = meta.runePriceUSD;

    // Insert into the database
    sqlx::query(
        r#"
        INSERT INTO Earning_Data_Rune_Pool_Meta (
            start_time, end_time, liquidity_fees, block_rewards, earnings, bonding_earnings, 
            liquidity_earnings, avg_node_count, rune_price_usd
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id
        "#,
    )
    .bind(meta_start_time)
    .bind(meta_end_time)
    .bind(meta_liquidity_fees)
    .bind(meta_block_rewards)
    .bind(meta_earnings)
    .bind(meta_bonding_earnings)
    .bind(meta_liquidity_earnings)
    .bind(avg_node_count)
    .bind(rune_price_usd)
    .fetch_one(pool)
    .await?;

    // Log insertion success
    std::println!("The data has been inserted into earning_data_rune_pool_meta");

    Ok(())
}

pub async fn insert_rune_pool_intervals(
    intervals: &[RunePoolInterval], // Accepts a slice of RunePoolInterval
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    for interval in intervals {
        // Convert interval.startTime to i64 with error handling
        let interval_start_time: i64 = interval.startTime.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "startTime {} too large for i64",
                interval.startTime
            ))
        })?;

        // Convert interval.endTime to i64 with error handling
        let interval_end_time: i64 = interval.endTime.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!("endTime {} too large for i64", interval.endTime))
        })?;

        // Convert interval.liquidityFees to i64 with error handling
        let interval_liquidity_fees: i64 = interval.liquidityFees.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "liquidityFees {} too large for i64",
                interval.liquidityFees
            ))
        })?;

        // Convert interval.blockRewards to i64 with error handling
        let interval_block_rewards: i64 = interval.blockRewards.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "blockRewards {} too large for i64",
                interval.blockRewards
            ))
        })?;

        // Convert interval.earnings to i64 with error handling
        let interval_earnings: i64 = interval.earnings.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!("earnings {} too large for i64", interval.earnings))
        })?;

        // Convert interval.bondingEarnings to i64 with error handling
        let interval_bonding_earnings: i64 = interval.bondingEarnings.try_into().map_err(|_| {
            sqlx::Error::Protocol(format!(
                "bondingEarnings {} too large for i64",
                interval.bondingEarnings
            ))
        })?;

        // Convert interval.liquidityEarnings to i64 with error handling
        let interval_liquidity_earnings: i64 =
            interval.liquidityEarnings.try_into().map_err(|_| {
                sqlx::Error::Protocol(format!(
                    "liquidityEarnings {} too large for i64",
                    interval.liquidityEarnings
                ))
            })?;

        // Use f64 directly for avgNodeCount and runePriceUSD as they do not need conversion
        let avg_node_count = interval.avgNodeCount;
        let rune_price_usd = interval.runePriceUSD;

        if !interval.pools.is_empty() {
            for pool_data in &interval.pools {
                // Insert pool data
                insert_pool_data(pool_data, pool).await?;
            }
        }

        // Insert into the database
        sqlx::query(
            r#"
            INSERT INTO Earning_Data_Rune_Pool_Interval (
                start_time, end_time, liquidity_fees, block_rewards, earnings, bonding_earnings, 
                liquidity_earnings, avg_node_count, rune_price_usd
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id
            "#,
        )
        .bind(interval_start_time)
        .bind(interval_end_time)
        .bind(interval_liquidity_fees)
        .bind(interval_block_rewards)
        .bind(interval_earnings)
        .bind(interval_bonding_earnings)
        .bind(interval_liquidity_earnings)
        .bind(avg_node_count)
        .bind(rune_price_usd)
        .fetch_one(pool)
        .await?;

        // Log insertion success
        std::println!("The data has been inserted into earning_data_rune_pool_interval");
    }

    Ok(())
}
