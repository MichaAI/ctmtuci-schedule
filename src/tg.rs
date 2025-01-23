use crate::dialoge::fetch::receive_group;
use crate::sheet_parser;
use crate::{dialoge::fetch, utils::parse_date};
use log::{debug, info};
use teloxide::{
    dispatching::UpdateFilterExt,
    dptree::{self, endpoint},
    macros::BotCommands,
    payloads::SendMessageSetters,
    prelude::{Dispatcher, Requester, ResponseResult},
    types::{InlineQuery, Message, Update},
    utils::command::BotCommands as _,
    Bot, RequestError,
};


pub async fn register(bot: Bot) {
    let handler = dptree::entry()
        .branch(Update::filter_message().branch(endpoint(answer)))
        .branch(Update::filter_inline_query().branch(endpoint(inline_query)))
        .branch(Update::filter_callback_query().endpoint(fetch::callback_handler));
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn answer(bot: Bot, msg: Message) -> ResponseResult<()> {
    let text = match msg.text() {
        Some(t) => t,
        None => {
            let _ = bot.send_message(msg.chat.id, "No text").await;
            return Ok(());
        }
    };
    debug!("Text: {}", text);
    let cmd = match Command::parse(text, "ctmutci_schedule_bot") {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };
    let _: Result<(), ()> = match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
            Ok(())
        }
        Command::Tomorrow(str) => {
            info!("String: {}", str);
            tokio::spawn(tomorrow(bot, msg, str));
            Ok(())
        }
        Command::Get { group, date } => {
            info!("Group: {}, Date: {}", group, date);
            tokio::spawn(get(bot, msg, group, date));
            Ok(())
        }
        Command::Fetch { group } => {
            info!("Group: {}", group);
            tokio::spawn(receive_group(bot, msg, group));
            Ok(())
        }
    };

    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "() - Help command")]
    Help,
    #[command(description = "(group) - Отображает расписание на завтра")]
    Tomorrow(String),
    #[command(
        description = "(group, date) - Отображает расписание на указанную дату",
        parse_with = "split"
    )]
    Get {
        group: String,
        date: String,
    },
    Fetch {
        group: String,
    },
}

/// This function is responsible for fetching and sending the schedule for the next day to a Telegram chat.
///
/// # Parameters
///
/// * `bot`: A `Bot` instance from the `teloxide` crate, used to send messages to the Telegram chat.
/// * `msg`: A `Message` instance representing the incoming message in the Telegram chat.
/// * `s`: A `String` representing the group for which the schedule needs to be fetched.
///
/// # Return
///
/// This function returns a `Result` with `Ok(())` if the schedule is successfully sent to the chat,
/// or `Err(e)` if an error occurs while fetching or sending the schedule.
async fn tomorrow(bot: Bot, msg: Message, s: String) {
    match sheet_parser::fetch_schedule(
        chrono::Local::now().date_naive() + chrono::Duration::days(1),
        s,
    )
    .await
    {
        Ok(s) => bot
            .send_message(msg.chat.id, s)
            .parse_mode(teloxide::types::ParseMode::Html)
            .await
            .unwrap(),
        Err(e) => bot
            .send_message(
                msg.chat.id,
                "Error occured while fetching schedule".to_owned() + &e,
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .await
            .unwrap(),
    };
}

async fn get(bot: Bot, msg: Message, group: String, date: String) {
    let date = match parse_date(&date) {
        Ok(parsed_date) => parsed_date,
        Err(_) => {
            bot.send_message(msg.chat.id, "Неверный формат даты")
                .await
                .unwrap();
            return ();
        }
    };

    match sheet_parser::fetch_schedule(date, group).await {
        Ok(s) => bot
            .send_message(msg.chat.id, s)
            .parse_mode(teloxide::types::ParseMode::Html)
            .await
            .unwrap(),
        Err(e) => bot
            .send_message(
                msg.chat.id,
                "Error occured while fetching schedule".to_owned() + &e,
            )
            .parse_mode(teloxide::types::ParseMode::Html)
            .await
            .unwrap(),
    };
}

async fn inline_query(_bot: Bot, _query: InlineQuery) -> Result<(), RequestError> {
    Ok(())
}