use crate::{
    datatypes::MessageMetadata, sheet_updater::GROOPS, sheet_parser::fetch_schedule, utils::get_calendar,
};
use log::{debug, info, trace, warn};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot, RequestError,
};

/// Sends a calendar to the user for selecting a date, based on the specified group.
/// 
/// This function checks if the provided group exists in the `GROOPS` data. If the group
/// is found, it generates a calendar and sends it to the user, allowing them to select a date.
/// 
/// # Parameters
/// 
/// - `bot`: The `Bot` instance used to send messages and interact with the Telegram API.
/// - `msg`: The `Message` object representing the incoming message from the user.
/// - `group`: A `String` representing the group name provided by the user.
/// 
/// # Returns
/// 
/// This function does not return a value. It sends a message to the user with a calendar
/// for date selection if the group is found, or a "Group not found" message if the group
/// does not exist.
pub async fn receive_group(bot: Bot, msg: Message, group: String) {
    trace!("Entering receive_group function with group: {}", group);
    let group_data = GROOPS.read().await;
    if !group_data.contains_key(&group.to_lowercase()) {
        warn!("Group not found: {}", group);
        bot.send_message(msg.chat.id, "Группа не найдена")
            .await
            .unwrap();
        return;
    }
    info!("Group found: {}", group);
    let (buttons, name) = get_calendar(None);
    debug!("Generated calendar for month: {}", name);
    let buttons: Vec<Vec<InlineKeyboardButton>> = buttons
        .iter()
        .map(|week| {
            week.iter()
                .map(|day| {
                    if day.is_some() {
                        trace!("Adding button for day: {}", day.unwrap().format("%d"));
                        InlineKeyboardButton::callback(
                            day.unwrap().format("%d").to_string(),
                            serde_json::to_string(&MessageMetadata {
                                group: group.to_string(),
                                date: day.unwrap(),
                            })
                            .unwrap(),
                        )
                    } else {
                        InlineKeyboardButton::callback(" ".to_string(), " ".to_string())
                    }
                })
                .collect()
        })
        .collect();

    info!("Sending calendar to user for group: {}", group);
    bot.send_message(
        msg.chat.id,
        format!("Выберите дату используя календарь ниже. Месяц: {}", name),
    )
    .reply_markup(InlineKeyboardMarkup::new(buttons))
    .await
    .unwrap();
    trace!("Exiting receive_group function");
}

pub async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), RequestError> {
    if let Some(data) = q.data {
        let data = serde_json::from_str::<MessageMetadata>(&data);

        if let Ok(data) = data {
            let _ = bot
                .send_message(
                    q.from.id,
                    fetch_schedule(data.date, data.group).await.unwrap(),
                )
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;

            let _ = bot.answer_callback_query(q.id).await;
        }
    }

    Ok(())
}
