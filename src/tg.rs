use crate::{sheet_updater::GROOPS, SHEET};
use calamine::{Data, Reader};
use chrono::Datelike;
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

const DAY_ROW: [u8; 6] = [7, 13, 19, 25, 31, 37];
pub async fn register(bot: Bot) {
    let handler = dptree::entry()
        .branch(Update::filter_message().branch(endpoint(answer)))
        .branch(Update::filter_inline_query().branch(endpoint(inline_query)));
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
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
    };

    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Отображает расписание на завтра")]
    Tomorrow(String),
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
    match fetch_schedule(
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

async fn inline_query(bot: Bot, query: InlineQuery) -> Result<(), RequestError> {
    Ok(())
}

async fn fetch_schedule(date: chrono::NaiveDate, group: String) -> Result<String, String> {
    if date.weekday().num_days_from_monday() == 6 {
        return Ok("Выходной".to_string());
    }
    let group_data = GROOPS.read().await;
    if !group_data.contains_key(&group.to_lowercase()) {
        return Err("Группа не найдена".to_string());
    }
    let (_, column_index) = group_data.get(&group).unwrap();

    let mut sheet_lock = SHEET.lock().await;
    let mut sheet = sheet_lock.as_mut();
    if let Some(sheet) = sheet {
        let monday_date = date - chrono::Duration::days(date.weekday().num_days_from_monday() as i64);
        let formatted_monday_date = monday_date.format("%d.%m").to_string();
        let mut sheet_names = sheet.sheet_names();
        let sheet_name = sheet_names
            .iter_mut()
            .find(|s| s.starts_with(formatted_monday_date.as_str()))
            .unwrap();
        let worksheet_data = sheet.worksheet_range(sheet_name).unwrap();
        let mut response = String::new();
        for lesson_index in 0..5 {
            let row_index = DAY_ROW[date.weekday().num_days_from_monday() as usize] + lesson_index;
            let mut cell_data = worksheet_data.get((row_index as usize, *column_index as usize));
            if !sheet_name.contains("не") {
                let possible_cell_data = worksheet_data.get((row_index as usize, *column_index as usize + 1));
                if let Some(possible_cell_data) = possible_cell_data {
                    match possible_cell_data {
                        Data::String(s) => {
                            if !s.is_empty() {
                                cell_data = Some(possible_cell_data);
                            }
                        }
                        Data::Empty => {}
                        _ => {}
                    }
                }
            }

            let cell_data = match cell_data {
                Some(cell_data) => cell_data,
                None => continue,
            };

            let time_data = worksheet_data.get((row_index as usize, 2));
            let time = match time_data {
                Some(Data::String(time)) => time,
                _ => "",
            };

            let classroom_data = worksheet_data.get((row_index as usize, *column_index + 2));
            let classroom = match classroom_data {
                Some(Data::String(classroom)) => classroom,
                _ => "",
            };

            match cell_data {
                Data::String(s) => {
                    if !s.is_empty() {
                        response.push_str(&format!(
                            "{}: Время: <b>{}</b>, Аудитория: <b>{}</b> <blockquote>{}</blockquote> \n",
                            lesson_index + 1,
                            time,
                            classroom,
                            s
                        ));
                    }
                }
                _ => {}
            }
        }
        return Ok(response);
    }
    Ok(String::new())
}
