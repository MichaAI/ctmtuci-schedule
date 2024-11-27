use bytes;
use calamine::Sheets;
use dotenvy::dotenv;
use log::*;
use sheet_updater::start_update;
use std::{
    io::Cursor,
    sync::{Arc, LazyLock},
};
use teloxide::prelude::*;

mod sheet_updater;
mod tg;
mod utils;
mod datatypes;
mod dialoge;
static SHEET: LazyLock<Arc<tokio::sync::Mutex<Box<Option<Sheets<Cursor<bytes::Bytes>>>>>>> =
    LazyLock::new(|| Arc::new(tokio::sync::Mutex::new(Box::new(None))));
const URI: &str = "https://docs.google.com/spreadsheets/d/1S3kj0zo_QDERJu7O2QU1J4gMRx-K381m/export";
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
