use axum::{Json, extract::State, http::StatusCode};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool, Row, postgres::PgRow};

#[derive(Serialize, Deserialize, Debug)]
pub struct EarningDataPoolData {
    id: i32,
    pool: String,
    asset_liquidity_fees: Option<i64>,
    rune_liquidity_fees: Option<i64>,
    total_liquidity_fees_rune: Option<i64>,
    saver_earning: Option<i64>,
    rewards: Option<i64>,
    earnings: Option<i64>,
}

impl FromRow<'_, PgRow> for EarningDataPoolData {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(EarningDataPoolData {
            id: row.try_get("id")?,
            pool: row.try_get("pool")?,
            asset_liquidity_fees: row.try_get("asset_liquidity_fees")?,
            rune_liquidity_fees: row.try_get("rune_liquidity_fees")?,
            total_liquidity_fees_rune: row.try_get("total_liquidity_fees_rune")?,
            saver_earning: row.try_get("saver_earning")?,
            rewards: row.try_get("rewards")?,
            earnings: row.try_get("earnings")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EarningDataRunePoolMeta {
    id: i32,
    start_time: Option<i64>,
    end_time: Option<i64>,
    liquidity_fees: Option<i64>,
    block_rewards: Option<i64>,
    earnings: Option<i64>,
    bonding_earnings: Option<i64>,
    liquidity_earnings: Option<i64>,
    avg_node_count: Option<f64>,
    rune_price_usd: Option<f64>,
}

impl FromRow<'_, PgRow> for EarningDataRunePoolMeta {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(EarningDataRunePoolMeta {
            id: row.try_get("id")?,
            start_time: row.try_get("start_time")?,
            end_time: row.try_get("end_time")?,
            liquidity_fees: row.try_get("liquidity_fees")?,
            block_rewards: row.try_get("block_rewards")?,
            earnings: row.try_get("earnings")?,
            bonding_earnings: row.try_get("bonding_earnings")?,
            liquidity_earnings: row.try_get("liquidity_earnings")?,
            avg_node_count: row.try_get("avg_node_count")?,
            rune_price_usd: row.try_get("rune_price_usd")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EarningDataRunePoolInterval {
    id: i32,
    start_time: Option<i64>,
    end_time: Option<i64>,
    liquidity_fees: Option<i64>,
    block_rewards: Option<i64>,
    earnings: Option<i64>,
    bonding_earnings: Option<i64>,
    liquidity_earnings: Option<i64>,
    avg_node_count: Option<f64>,
    rune_price_usd: Option<f64>,
    pools: Vec<EarningDataPoolData>,
}

impl FromRow<'_, PgRow> for EarningDataRunePoolInterval {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let pools: Vec<EarningDataPoolData> = serde_json::from_value(row.try_get("pools")?).unwrap_or_default();
        
        Ok(EarningDataRunePoolInterval {
            id: row.try_get("id")?,
            start_time: row.try_get("start_time")?,
            end_time: row.try_get("end_time")?,
            liquidity_fees: row.try_get("liquidity_fees")?,
            block_rewards: row.try_get("block_rewards")?,
            earnings: row.try_get("earnings")?,
            bonding_earnings: row.try_get("bonding_earnings")?,
            liquidity_earnings: row.try_get("liquidity_earnings")?,
            avg_node_count: row.try_get("avg_node_count")?,
            rune_price_usd: row.try_get("rune_price_usd")?,
            pools,
        })
    }
}

pub async fn fetch_pool_data(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EarningDataPoolData>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, EarningDataPoolData>(
        "SELECT * FROM earning_data_pool_data"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching pool data: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok(Json(rows))
}

pub async fn fetch_meta(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EarningDataRunePoolMeta>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, EarningDataRunePoolMeta>(
        "SELECT * FROM earning_data_rune_pool_meta"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching meta: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok(Json(rows))
}

pub async fn fetch_intervals(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EarningDataRunePoolInterval>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, EarningDataRunePoolInterval>(
        r#"
        WITH pool_data AS (
            SELECT json_agg(
                json_build_object(
                    'id', p.id,
                    'pool', p.pool,
                    'asset_liquidity_fees', p.asset_liquidity_fees,
                    'rune_liquidity_fees', p.rune_liquidity_fees,
                    'total_liquidity_fees_rune', p.total_liquidity_fees_rune,
                    'saver_earning', p.saver_earning,
                    'rewards', p.rewards,
                    'earnings', p.earnings
                )
            ) as pools_data
            FROM earning_data_pool_data p
        )
        SELECT 
            i.*,
            COALESCE(pd.pools_data, '[]'::json) as pools
        FROM earning_data_rune_pool_interval i
        CROSS JOIN pool_data pd
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching intervals: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok(Json(rows))
}
