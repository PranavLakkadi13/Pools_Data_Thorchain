// data_structs/depth_data.rs
use axum::http::StatusCode;
use axum::{extract::State, Json};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize}; // query_data_from_db/rune_pool_data_query.rs
use sqlx::PgPool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct RunePoolDepthMeta {
    pub id: i32,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub price_shift_loss: Option<BigDecimal>, // Nullable field
    pub luvi_increase: Option<BigDecimal>,    // Nullable field
    pub start_asset_depth: Option<i64>,
    pub start_rune_depth: Option<i64>,
    pub start_lp_units: Option<i64>,
    pub start_member_count: Option<i64>,
    pub start_synth_units: Option<i64>,
    pub end_asset_depth: Option<i64>,
    pub end_rune_depth: Option<i64>,
    pub end_lp_units: Option<i64>,
    pub end_member_count: Option<i64>,
    pub end_synth_units: Option<i64>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct RunePoolDepthIntervals {
    pub id: i32,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub asset_depth: Option<i64>,
    pub rune_depth: Option<i64>,
    pub asset_price: Option<BigDecimal>,     // Nullable field
    pub asset_price_usd: Option<BigDecimal>, // Nullable field
    pub liquidity_units: Option<i64>,
    pub members_count: Option<i64>,
    pub synth_units: Option<i64>,
    pub synth_supply: Option<i64>,
    pub units: Option<i64>,
    pub luvi: Option<BigDecimal>, // Nullable field
}

pub async fn query_meta(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<RunePoolDepthMeta>>, (StatusCode, String)> {
    let rows = sqlx::query_as!(
        RunePoolDepthMeta,
        r#"
        SELECT 
            id, 
            startTime as start_time, 
            endTime as end_time, 
            COALESCE(
                NULLIF(priceShiftLoss::numeric::text, 'NaN')::numeric, 
                NULL
            ) as "price_shift_loss!: Option<BigDecimal>",
            COALESCE(
                NULLIF(luviIncrease::numeric::text, 'NaN')::numeric,
                NULL
            ) as "luvi_increase!: Option<BigDecimal>",
            startAssetDepth as start_asset_depth,
            startRuneDepth as start_rune_depth,
            startLPUnits as start_lp_units,
            startMemberCount as start_member_count,
            startSynthUnits as start_synth_units,
            endAssetDepth as end_asset_depth,
            endRuneDepth as end_rune_depth,
            endLPUnits as end_lp_units,
            endMemberCount as end_member_count,
            endSynthUnits as end_synth_units
        FROM Rune_Pool_Depth_Meta
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok(Json(rows))
}

pub async fn query_intervals(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<RunePoolDepthIntervals>>, (StatusCode, String)> {
    let rows = sqlx::query_as!(
        RunePoolDepthIntervals,
        r#"
        SELECT 
            id, 
            startTime as start_time, 
            endTime as end_time, 
            assetDepth as asset_depth, 
            runeDepth as rune_depth, 
            COALESCE(
                NULLIF(assetPrice::numeric::text, 'NaN')::numeric,
                NULL
            ) as "asset_price!: Option<BigDecimal>",
            COALESCE(
                NULLIF(assetPriceUSD::numeric::text, 'NaN')::numeric,
                NULL
            ) as "asset_price_usd!: Option<BigDecimal>",
            liquidityUnits as liquidity_units,
            membersCount as members_count,
            synthUnits as synth_units,
            synthSupply as synth_supply,
            units as units,
            COALESCE(
                NULLIF(luvi::numeric::text, 'NaN')::numeric,
                NULL
            ) as "luvi!: Option<BigDecimal>"
        FROM Rune_Pool_Depth_Intervals
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    std::println!("The query has been completed successfully! depth data intervals");
    Ok(Json(rows))
}
