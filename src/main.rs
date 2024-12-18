use dotenv::dotenv;
use serde_json;
use sqlx::PgPool;
use std::env;

mod data_structs;
use data_structs::depth_data::RootDepthDetails;
use data_structs::earning_history::RootEarnDetails;
use data_structs::rune_pool::RunePoolIntervalsInt;
use data_structs::swap_history::RootSwapDetails;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // Create tables first
    data_structs::rune_pool::create_tables(&pool).await?;
    // Fetch and process RunePool data
    let runepool_data = runepool_fn().await?.text().await?;
    let runepool_parsed = serde_json::from_str::<RunePoolIntervalsInt>(&runepool_data)?;

    // Insert RunePool data
    data_structs::rune_pool::insert_data(&pool, runepool_parsed).await?;

    // Fetch and parse other data (optional)
    // let swaphistory = swap_history().await?.text().await?;
    // let x = serde_json::from_str::<RootSwapDetails>(&swaphistory)?;
    // println!("{:#?}", x);

    // let earning_data = earning_history().await?.text().await?;
    // let x = serde_json::from_str::<RootEarnDetails>(&earning_data)?;
    // let y = x.intervals;
    // println!("the pool earning swap details {:#?}", y[0]);

    // let depth_data = depth_data().await?.text().await?;
    // let x = serde_json::from_str::<RootDepthDetails>(&depth_data)?;
    // println!("{:#?}", x);

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
