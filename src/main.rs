use dotenv::dotenv;
use serde_json;
use sqlx::PgPool;
use std::env;

mod data_structs;
use data_structs::depth_data::RootDepthDetails;
use data_structs::earning_history::RootEarnDetails;
use data_structs::rune_pool::RunePoolIntervalsInt;
use data_structs::swap_history::RootSwapDetails;

mod insert_data_post_migration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // creates the sql tables in the db
    let _ = migration_script().await?;

    // let mut count = 0u8;

    // while count < 5 {
    #[allow(unused_doc_comments)]
    /////////////////////////////////////////////////////
    /// Rune POOL DATA INSERTION  SCRIPT ///////////////
    ////////////////////////////////////////////////////
    let runepool_data = runepool_fn().await?.text().await?;
    let runepool_parsed = serde_json::from_str::<RunePoolIntervalsInt>(&runepool_data)?;
    std::println!("The parsed data {:#?}", runepool_parsed);

    std::println!("the rune pool to be updated!!");
    insert_data_post_migration::rune_pool_data_insert_script::insert_data(&pool, runepool_parsed)
        .await?;

    #[allow(unused_doc_comments)]
    /////////////////////////////////////////////////////
    /// DEPTH DATA INSERTION SCRIPT /////////////////////
    ////////////////////////////////////////////////////
    let depth_data = depth_data().await?.text().await?;
    std::println!("The depth data {:#?}", depth_data);
    let depth_parsed = serde_json::from_str::<RootDepthDetails>(&depth_data)?;
    std::println!("The parsed depth data {:#?}", depth_parsed);

    insert_data_post_migration::depth_data_insert_script::insert_data(&pool, depth_parsed).await?;

    #[allow(unused_doc_comments)]
    /////////////////////////////////////////////////////
    /// EARNING DATA INSERTION SCRIPT ///////////////////
    ////////////////////////////////////////////////////
    // let earning_data = earning_history().await?.text().await?;

    // std::println!("Hello2");
    // let earning_data_parsed = serde_json::from_str::<RootEarnDetails>(&earning_data)?;
    // std::println!("Hello3");
    // let y = earning_data_parsed;

    // let _ = insert_data_post_migration::earning_data_insert_script::insert_rune_pool_meta(
    //     &y.meta,&pool
    // )
    // .await?;
    // let _ = insert_data_post_migration::earning_data_insert_script::insert_rune_pool_intervals(&y.intervals, &pool)
    // .await?;


    #[allow(unused_doc_comments)]
    /////////////////////////////////////////////////////
    /// SWAP DATA INSERTION SCRIPT /////////////////////
    ////////////////////////////////////////////////////
    let swap_history = swap_history().await?.text().await?;
    let swap_history_parsed = serde_json::from_str::<RootSwapDetails>(&swap_history)?;
    let z = swap_history_parsed;
    let _ =
        insert_data_post_migration::swap_data_insert_script::insert_rune_pool_meta(&z.meta, &pool)
            .await?;
    let _ = insert_data_post_migration::swap_data_insert_script::insert_rune_pool_intervals(
        &z.intervals,
        &pool,
    )
    .await?;

    // count += 1;
    // }

    Ok(())
}

async fn runepool_fn() -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get(
        "https://midgard.ninerealms.com/v2/history/runepool?interval=day&count=10",
    )
    .await
}

async fn swap_history() -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get(
        "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&count=10",
    )
    .await
}

async fn earning_history() -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get("https://midgard.ninerealms.com/v2/history/earnings?interval=hour&count=10").await
}

async fn depth_data() -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get("https://midgard.ninerealms.com/v2/history/depths/AVAX.AVAX/?interval=hour&count=10").await
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
