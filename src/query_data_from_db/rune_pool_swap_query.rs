use axum::{Json, extract::State, http::StatusCode};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool, Row, postgres::PgRow};

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapDataRunePoolMeta {
    id: i32,
    start_time: Option<i64>,
    end_time: Option<i64>,
    to_asset_count: Option<i64>,
    to_rune_count: Option<i64>,
    to_trade_count: Option<i64>,
    from_trade_count: Option<i64>,
    synth_mint_count: Option<i64>,
    synth_redeem_count: Option<i64>,
    total_count: Option<i64>,
    to_asset_volume: Option<i64>,
    to_rune_volume: Option<i64>,
    to_trade_volume: Option<i64>,
    from_trade_volume: Option<i64>,
    synth_mint_volume: Option<i64>,
    synth_redeem_volume: Option<i64>,
    total_volume: Option<i64>,
    to_asset_volume_usd: Option<i64>,    // changed from f64 to i64
    to_rune_volume_usd: Option<i64>,     // changed from f64 to i64
    to_trade_volume_usd: Option<i64>,    // changed from f64 to i64
    from_trade_volume_usd: Option<i64>,  // changed from f64 to i64
    synth_mint_volume_usd: Option<i64>,  // changed from f64 to i64
    synth_redeem_volume_usd: Option<i64>,// changed from f64 to i64
    total_volume_usd: Option<i64>,       // changed from f64 to i64
    to_asset_fees: Option<i64>,
    to_rune_fees: Option<i64>,
    to_trade_fees: Option<i64>,
    from_trade_fees: Option<i64>,
    synth_mint_fees: Option<i64>,
    synth_redeem_fees: Option<i64>,
    total_fees: Option<i64>,
    to_asset_average_slip: Option<f64>,
    to_rune_average_slip: Option<f64>,
    to_trade_average_slip: Option<f64>,
    from_trade_average_slip: Option<f64>,
    synth_mint_average_slip: Option<f64>,
    synth_redeem_average_slip: Option<f64>,
    average_slip: Option<f64>,
    rune_price_usd: Option<f64>,
}

impl FromRow<'_, PgRow> for SwapDataRunePoolMeta {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(SwapDataRunePoolMeta {
            id: row.try_get("id")?,
            start_time: row.try_get("start_time")?,
            end_time: row.try_get("end_time")?,
            to_asset_count: row.try_get("to_asset_count")?,
            to_rune_count: row.try_get("to_rune_count")?,
            to_trade_count: row.try_get("to_trade_count")?,
            from_trade_count: row.try_get("from_trade_count")?,
            synth_mint_count: row.try_get("synth_mint_count")?,
            synth_redeem_count: row.try_get("synth_redeem_count")?,
            total_count: row.try_get("total_count")?,
            to_asset_volume: row.try_get("to_asset_volume")?,
            to_rune_volume: row.try_get("to_rune_volume")?,
            to_trade_volume: row.try_get("to_trade_volume")?,
            from_trade_volume: row.try_get("from_trade_volume")?,
            synth_mint_volume: row.try_get("synth_mint_volume")?,
            synth_redeem_volume: row.try_get("synth_redeem_volume")?,
            total_volume: row.try_get("total_volume")?,
            to_asset_volume_usd: row.try_get("to_asset_volume_usd")?,
            to_rune_volume_usd: row.try_get("to_rune_volume_usd")?,
            to_trade_volume_usd: row.try_get("to_trade_volume_usd")?,
            from_trade_volume_usd: row.try_get("from_trade_volume_usd")?,
            synth_mint_volume_usd: row.try_get("synth_mint_volume_usd")?,
            synth_redeem_volume_usd: row.try_get("synth_redeem_volume_usd")?,
            total_volume_usd: row.try_get("total_volume_usd")?,
            to_asset_fees: row.try_get("to_asset_fees")?,
            to_rune_fees: row.try_get("to_rune_fees")?,
            to_trade_fees: row.try_get("to_trade_fees")?,
            from_trade_fees: row.try_get("from_trade_fees")?,
            synth_mint_fees: row.try_get("synth_mint_fees")?,
            synth_redeem_fees: row.try_get("synth_redeem_fees")?,
            total_fees: row.try_get("total_fees")?,
            to_asset_average_slip: row.try_get("to_asset_average_slip")?,
            to_rune_average_slip: row.try_get("to_rune_average_slip")?,
            to_trade_average_slip: row.try_get("to_trade_average_slip")?,
            from_trade_average_slip: row.try_get("from_trade_average_slip")?,
            synth_mint_average_slip: row.try_get("synth_mint_average_slip")?,
            synth_redeem_average_slip: row.try_get("synth_redeem_average_slip")?,
            average_slip: row.try_get("average_slip")?,
            rune_price_usd: row.try_get("rune_price_usd")?,
        })
    }
}

pub async fn fetch_meta(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<SwapDataRunePoolMeta>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, SwapDataRunePoolMeta>(
        "SELECT * FROM swap_data_rune_pool_meta"
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapDataRunePoolInterval {
    id: i32,
    start_time: Option<i64>,
    end_time: Option<i64>,
    to_asset_count: Option<i64>,
    to_rune_count: Option<i64>,
    to_trade_count: Option<i64>,
    from_trade_count: Option<i64>,
    synth_mint_count: Option<i64>,
    synth_redeem_count: Option<i64>,
    total_count: Option<i64>,
    to_asset_volume: Option<i64>,
    to_rune_volume: Option<i64>,
    to_trade_volume: Option<i64>,
    from_trade_volume: Option<i64>,
    synth_mint_volume: Option<i64>,
    synth_redeem_volume: Option<i64>,
    total_volume: Option<i64>,
    to_asset_average_slip: Option<f64>,
    to_rune_average_slip: Option<f64>,
    average_slip: Option<f64>,
    rune_price_usd: Option<f64>,
}

impl FromRow<'_, PgRow> for SwapDataRunePoolInterval {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(SwapDataRunePoolInterval {
            id: row.try_get("id")?,
            start_time: row.try_get("start_time")?,
            end_time: row.try_get("end_time")?,
            to_asset_count: row.try_get("to_asset_count")?,
            to_rune_count: row.try_get("to_rune_count")?,
            to_trade_count: row.try_get("to_trade_count")?,
            from_trade_count: row.try_get("from_trade_count")?,
            synth_mint_count: row.try_get("synth_mint_count")?,
            synth_redeem_count: row.try_get("synth_redeem_count")?,
            total_count: row.try_get("total_count")?,
            to_asset_volume: row.try_get("to_asset_volume")?,
            to_rune_volume: row.try_get("to_rune_volume")?,
            to_trade_volume: row.try_get("to_trade_volume")?,
            from_trade_volume: row.try_get("from_trade_volume")?,
            synth_mint_volume: row.try_get("synth_mint_volume")?,
            synth_redeem_volume: row.try_get("synth_redeem_volume")?,
            total_volume: row.try_get("total_volume")?,
            to_asset_average_slip: row.try_get("to_asset_average_slip")?,
            to_rune_average_slip: row.try_get("to_rune_average_slip")?,
            average_slip: row.try_get("average_slip")?,
            rune_price_usd: row.try_get("rune_price_usd")?,
        })
    }
}

pub async fn fetch_intervals(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<SwapDataRunePoolInterval>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, SwapDataRunePoolInterval>(
        "SELECT * FROM swap_data_rune_pool_interval"
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