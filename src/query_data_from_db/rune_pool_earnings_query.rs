// use axum::{extract::State, Json};
// use axum::http::StatusCode;
// use sqlx::{FromRow, PgPool};
// use serde::{Deserialize, Serialize};
// use bigdecimal::BigDecimal;

// #[derive(Serialize, Deserialize, FromRow, Clone)]
// struct PoolData {
//     pool: String,
//     #[serde(default)]
//     asset_liquidity_fees: Option<BigDecimal>,
//     #[serde(default)]
//     rune_liquidity_fees: Option<BigDecimal>,
//     #[serde(default)]
//     total_liquidity_fees_rune: Option<BigDecimal>,
//     #[serde(default)]
//     saver_earning: Option<BigDecimal>,
//     #[serde(default)]
//     rewards: Option<BigDecimal>,
//     #[serde(default)]
//     earnings: Option<BigDecimal>,
// }

// #[derive(Serialize, Deserialize, FromRow)]
// struct IntervalData {
//     start_time: i64,
//     end_time: i64,
//     #[serde(default)]
//     liquidity_fees: Option<BigDecimal>,
//     #[serde(default)]
//     block_rewards: Option<BigDecimal>,
//     #[serde(default)]
//     earnings: Option<BigDecimal>,
//     #[serde(default)]
//     bonding_earnings: Option<BigDecimal>,
//     #[serde(default)]
//     liquidity_earnings: Option<BigDecimal>,
//     avg_node_count: Option<f64>,
//     rune_price_usd: Option<f64>,
//     pools: Vec<PoolData>,
// }

// #[derive(Serialize)]
// struct IntervalsResponse {
//     intervals: Vec<IntervalData>,
// }

// pub async fn get_intervals(
//     State(pool): State<PgPool>
// ) -> Result<Json<IntervalsResponse>, (StatusCode, String)> {
//     let pool_data = sqlx::query_as!(
//         PoolData,
//         r#"
//         SELECT 
//             pool,
//             COALESCE(NULLIF(asset_liquidity_fees::numeric::text, 'NaN')::numeric, NULL) as "asset_liquidity_fees?: BigDecimal",
//             COALESCE(NULLIF(rune_liquidity_fees::numeric::text, 'NaN')::numeric, NULL) as "rune_liquidity_fees?: BigDecimal",
//             COALESCE(NULLIF(total_liquidity_fees_rune::numeric::text, 'NaN')::numeric, NULL) as "total_liquidity_fees_rune?: BigDecimal",
//             COALESCE(NULLIF(saver_earning::numeric::text, 'NaN')::numeric, NULL) as "saver_earning?: BigDecimal",
//             COALESCE(NULLIF(rewards::numeric::text, 'NaN')::numeric, NULL) as "rewards?: BigDecimal",
//             COALESCE(NULLIF(earnings::numeric::text, 'NaN')::numeric, NULL) as "earnings?: BigDecimal"
//         FROM Earning_Data_Pool_Data
//         "#
//     )
//     .fetch_all(&pool)
//     .await
//     .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

//     let intervals = sqlx::query_as!(
//         IntervalData,
//         r#"
//         SELECT 
//             start_time,
//             end_time,
//             COALESCE(NULLIF(liquidity_fees::numeric::text, 'NaN')::numeric, NULL) as "liquidity_fees?: BigDecimal",
//             COALESCE(NULLIF(block_rewards::numeric::text, 'NaN')::numeric, NULL) as "block_rewards?: BigDecimal",
//             COALESCE(NULLIF(earnings::numeric::text, 'NaN')::numeric, NULL) as "earnings?: BigDecimal",
//             COALESCE(NULLIF(bonding_earnings::numeric::text, 'NaN')::numeric, NULL) as "bonding_earnings?: BigDecimal",
//             COALESCE(NULLIF(liquidity_earnings::numeric::text, 'NaN')::numeric, NULL) as "liquidity_earnings?: BigDecimal",
//             avg_node_count,
//             rune_price_usd
//         FROM Earning_Data_Rune_Pool_Interval
//         "#
//     )
//     .fetch_all(&pool)
//     .await
//     .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

//     let intervals_with_pools = intervals
//         .into_iter()
//         .map(|interval| IntervalData {
//             pools: pool_data.clone(),
//             ..interval
//         })
//         .collect();

//     Ok(Json(IntervalsResponse {
//         intervals: intervals_with_pools,
//     }))
// }
