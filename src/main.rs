use dotenvy::dotenv;
use log::*;
use sheet_updater::start_update;
use teloxide::prelude::*;

mod sheet_updater;
mod tg;
mod utils;
mod datatypes;
mod dialoge;
mod sheet_parser;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();

    info!("Starting throw dice bot...");

    let _ = start_update().await;

    let bot = Bot::from_env();
    
    tg::register(bot).await;

    Ok(())
}
