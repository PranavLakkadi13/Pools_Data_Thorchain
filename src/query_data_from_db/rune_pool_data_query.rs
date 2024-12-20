use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct RunePoolMeta {
    id: i32,
    start_time: Option<i64>,
    end_time: Option<i64>,
    start_units: Option<i64>,
    start_count: Option<i64>,
    end_units: Option<i64>,
    end_count: Option<i64>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct RunePoolIntervals {
    id: i32,
    start_time: Option<i64>,
    end_time: Option<i64>,
    count: Option<i64>,
    units: Option<i64>,
}

#[derive(Deserialize)]
pub struct IntervalQueryParams {
    interval: Option<String>,
    count: Option<u32>,
    to: Option<i64>,
}

// pub async fn start_server(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
//     let app = Router::new()
//         .route("/meta", get(query_meta))
//         .route("/intervals", get(query_intervals))
//         .with_state(pool);

//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     println!("API Server running at http://{}", addr);

//     let listener = TcpListener::bind(addr).await?;
//     axum::serve(listener, app).await?;

//     Ok(())
// }

pub async fn query_meta(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<RunePoolMeta>>, (axum::http::StatusCode, String)> {
    let rows = sqlx::query_as!(
        RunePoolMeta,
        r#"
        SELECT 
            id, 
            startTime as start_time, 
            endTime as end_time, 
            startUnits as start_units,
            startCount as start_count, 
            endUnits as end_units, 
            endCount as end_count
        FROM Rune_Pool_Data_Meta
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok(Json(rows))
}

pub async fn query_intervals(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<RunePoolIntervals>>, (axum::http::StatusCode, String)> {
    let rows = sqlx::query_as!(
        RunePoolIntervals,
        r#"
        SELECT 
            id,
            startTime as start_time,
            endTime as end_time,
            count,
            units
        FROM rune_pool_data_intervals
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok(Json(rows))
}

// async fn query_intervals(
//     State(pool): State<PgPool>,
//     Query(params): Query<IntervalQueryParams>,
// ) -> Result<Json<Vec<RunePoolIntervals>>, (axum::http::StatusCode, String)> {
//     // Check if `to` parameter is provided
//     let to = params.to.ok_or((
//         axum::http::StatusCode::BAD_REQUEST,
//         "Missing 'to' timestamp".to_string(),
//     ))?;

//     // Determine the interval multiplier in seconds
//     let multiplier = match params.interval.as_deref() {
//         Some("hour") => 3600,
//         Some("day") => 86400,
//         Some("second") | None => 1, // Default to seconds if unspecified
//         Some(_) => {
//             return Err((
//                 axum::http::StatusCode::BAD_REQUEST,
//                 "Invalid interval value. Use 'second', 'hour', or 'day'.".to_string(),
//             ));
//         }
//     };

//     let count = params.count.unwrap_or(10); // Default to 10 rows if not provided

//     // Calculate the start range based on 'to' and the interval
//     let start_range = to - multiplier * (count as i64); // adjust based on interval

//     // Query the database for the filtered data
//     let rows = sqlx::query_as!(
//         RunePoolIntervals,
//         r#"
//         SELECT
//             id,
//             startTime AS start_time,
//             endTime AS end_time,
//             count,
//             units
//         FROM rune_pool_data_intervals
//         WHERE startTime >= $1 AND endTime <= $2
//         LIMIT $3
//         "#,
//         start_range,  // Filter rows where the startTime is within the calculated range
//         to,            // Filter rows where the endTime is within the provided 'to' timestamp
//         count as i64   // Limit the number of rows returned
//     )
//     .fetch_all(&pool)
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         (
//             axum::http::StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Database error: {}", e),
//         )
//     })?;

//     Ok(Json(rows))
// }
