use serde::{Deserialize, Deserializer};
use sqlx::PgPool;
use std::str::FromStr;

// Custom function to deserialize a string into a u128
fn string_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    u128::from_str(s).map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Debug)]
struct RunePoolIntervalsPriv {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub count: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub units: u128, // Converted to u128
}

#[derive(Deserialize, Debug)]
pub struct RunePoolIntervalsInt {
    pub intervals: Vec<RunePoolIntervalsPriv>,
    pub meta: RunePoolMeta,
}

#[derive(Deserialize, Debug)]
struct RunePoolMeta {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub startUnits: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub startCount: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub endUnits: u128, // Converted to u128
    #[serde(deserialize_with = "string_to_u128")]
    pub endCount: u128, // Converted to u128
}

pub async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS RunePoolMeta (
        id SERIAL PRIMARY KEY,
        startTime BIGINT,
        endTime BIGINT,
        startUnits BIGINT,
        startCount BIGINT,
        endUnits BIGINT,
        endCount BIGINT
    );"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS RunePoolIntervals (
        id SERIAL PRIMARY KEY,
        startTime BIGINT,
        endTime BIGINT,
        count BIGINT,
        units BIGINT
    );
    "#,
    )
    .execute(pool)
    .await?;

    std::println!("Tables created successfully!");

    Ok(())
}

pub async fn insert_data(pool: &PgPool, data: RunePoolIntervalsInt) -> Result<(), sqlx::Error> {
    // Convert meta data with overflow check
    let meta_start_time: i64 = data.meta.startTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("StartTime {} too large for i64", data.meta.startTime).into())
    })?;
    let meta_end_time: i64 = data.meta.endTime.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("EndTime {} too large for i64", data.meta.endTime).into())
    })?;
    let meta_start_units: i64 = data.meta.startUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!("StartUnits {} too large for i64", data.meta.startUnits).into(),
        )
    })?;
    let meta_start_count: i64 = data.meta.startCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(
            format!("StartCount {} too large for i64", data.meta.startCount).into(),
        )
    })?;
    let meta_end_units: i64 = data.meta.endUnits.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("EndUnits {} too large for i64", data.meta.endUnits).into())
    })?;
    let meta_end_count: i64 = data.meta.endCount.try_into().map_err(|_| {
        sqlx::Error::Protocol(format!("EndCount {} too large for i64", data.meta.endCount).into())
    })?;

    // Insert into RunePoolMeta
    sqlx::query(
        r#"
        INSERT INTO RunePoolMeta (startTime, endTime, startUnits, startCount, endUnits, endCount)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(meta_start_time)
    .bind(meta_end_time)
    .bind(meta_start_units)
    .bind(meta_start_count)
    .bind(meta_end_units)
    .bind(meta_end_count)
    .execute(pool)
    .await?;

    // Insert intervals into RunePoolIntervals
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
            INSERT INTO RunePoolIntervals (startTime, endTime, count, units)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(start_time)
        .bind(end_time)
        .bind(count)
        .bind(units)
        .execute(pool)
        .await?;
    }

    println!("Data inserted successfully!");

    Ok(())
}
