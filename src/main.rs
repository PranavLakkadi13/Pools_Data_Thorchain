use dotenv::dotenv;
use serde_json;
use sqlx::PgPool;
use std::env;

mod data_structs;
use data_structs::depth_data::RootDepthDetails;
use data_structs::earning_history::RootEarnDetails;
use data_structs::rune_pool::RunePoolIntervalsInt;
use data_structs::swap_history::{self, RootSwapDetails};

mod insert_data_post_migration;
use insert_data_post_migration::earning_data_insert_script;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // creates the sql tables in the db
    let _ = migration_script().await?;

    // Fetch and process RunePool data
    let runepool_data = runepool_fn().await?.text().await?;
    let runepool_parsed = serde_json::from_str::<RunePoolIntervalsInt>(&runepool_data)?;
    std::println!("The parsed data {:#?}", runepool_parsed);

    // Insert RunePool data
    data_structs::rune_pool::insert_data(&pool, runepool_parsed).await?;

    // Fetch and parse other data (optional)
    // let swaphistory = swap_history().await?.text().await?;
    // let x = serde_json::from_str::<RootSwapDetails>(&swaphistory)?;
    // println!("{:#?}", x);

    // data_structs::depth_data::create_tables(&pool).await?;

    let depth_data = depth_data().await?.text().await?;
    std::println!("The depth data {:#?}", depth_data);
    let depth_parsed = serde_json::from_str::<RootDepthDetails>(&depth_data)?;
    std::println!("The parsed depth data {:#?}", depth_parsed);

    data_structs::depth_data::insert_data(&pool, depth_parsed).await?;

    let earning_data = earning_history().await?.text().await?;
    let earning_data_parsed = serde_json::from_str::<RootEarnDetails>(&earning_data)?;
    let y = earning_data_parsed;
    std::println!("The parsed earning data {:#?}", y.meta);

    let _ = insert_data_post_migration::earning_data_insert_script::insert_rune_pool_meta(&y.meta, &pool).await?;
    let _ = insert_data_post_migration::earning_data_insert_script::insert_rune_pool_interval(&y.intervals[0], &pool).await?;

    std::println!("The pools data {:#?}", y.intervals[0].pools.len());
    // let _ = insert_data_post_migration::earning_data_script::insert_pool_data(&y.intervals[0].pools[0], &pool).await?;

    let swap_history = swap_history().await?.text().await?;
    let swap_history_parsed = serde_json::from_str::<RootSwapDetails>(&swap_history)?;
    let z = swap_history_parsed;
    let _ = insert_data_post_migration::swap_data_insert_script::insert_rune_pool_meta(&z.meta, &pool).await?;
    let _ = insert_data_post_migration::swap_data_insert_script::insert_rune_pool_interval(&z.intervals[0], &pool).await?;
    
    Ok(())
}

async fn runepool_fn() -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get("https://midgard.ninerealms.com/v2/history/runepool?interval=hour&count=1").await
}

async fn swap_history() -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get("https://midgard.ninerealms.com/v2/history/swaps?interval=day&count=1").await
}

async fn earning_history() -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get("https://midgard.ninerealms.com/v2/history/earnings?interval=hour&count=1").await
}

async fn depth_data() -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get("https://midgard.ninerealms.com/v2/history/depths/AVAX.AVAX").await
}

async fn migration_script() -> Result<(), sqlx::Error> {
    let pool =
        PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await?;

    let migrator = sqlx::migrate!("./migrations");
    // let pool = sqlx::postgres::PgPoolOptions::new();
    migrator.run(&pool).await?;

    std::println!("Migration script ran successfully!");

    Ok(())
}
