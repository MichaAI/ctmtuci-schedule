use calamine::{Data, DataType, Reader};
use chrono::Datelike;
use log::{error, trace};

use crate::sheet_updater::{GROOPS, SHEET};

const DAY_ROW: [u8; 6] = [7, 13, 19, 25, 31, 37];
/// Fetches and returns the schedule for the specified group and date.
///
/// If the date is Sunday, returns "Выходной".
/// If the group is not found, returns an error.
/// If the worksheet is not found, returns an error.
pub async fn fetch_schedule(date: chrono::NaiveDate, group: String) -> Result<String, String> {
    if date.weekday().num_days_from_monday() == 6 {
        return Ok("Выходной".to_string());
    }

    let group_data = GROOPS.read().await;
    if !group_data.contains_key(&group.to_lowercase()) {
        trace!("Group not found: {}", group);
        return Err("Группа не найдена".to_string());
    }

    let mut sheet_lock = SHEET.lock().await;
    let sheet = sheet_lock.as_mut();
    if sheet.is_none() {
        error!("Sheet is not found");
        return Err("500 Internal Server Error".to_string());
    }

    let sheet = sheet.as_mut().ok_or_else(|| {
        log::error!("Sheet is not found");
        "500 Internal Server Error"
    })?;

    let monday_date = date - chrono::Duration::days(date.weekday().num_days_from_monday() as i64);
    let formatted_monday_date = monday_date.format("%d.%m").to_string();

    let mut sheet_names = sheet.sheet_names();
    let sheet_name = sheet_names
        .iter_mut()
        .find(|s| s.contains(formatted_monday_date.as_str()))
        .ok_or_else(|| {
            log::error!("Sheet name not found: {}", formatted_monday_date);
            "500 Internal Server Error"
        })?;

    let worksheet_data = sheet
        .worksheet_range(sheet_name)
        .or(Err("500 Internal Server Error"))?;

    
    //Очень странная логика ниже
    let (_, column_index) = group_data.get(&group.to_lowercase()).ok_or_else(|| {
        log::error!("Group not found: {}", group);
        "500 Internal Server Error"
    })?;

    let column_index = *column_index as usize;


    let mut response = "".to_string();
    //Цикл для итерации по парам дня
    for lesson_index in 0..5 {
        //Находим индекс строки беря смещение по таблице в зависимости от дня недели и прибавляя смещение от итератора
        let row_index =
            (DAY_ROW[date.weekday().num_days_from_monday() as usize] + lesson_index) as usize;

        let mut cell = worksheet_data.get((row_index, column_index));

        //Условие того что неделя четная
        if !sheet_name.contains("не") {
            let next_cell = worksheet_data.get((row_index, column_index + 1));
            if let Some(next_cell_data) = next_cell {
                if !next_cell_data.is_empty() {
                    cell = Some(next_cell_data);
                }
            }
        }

        let lesson: &String = match cell {
            Some(Data::String(data)) => data,
            _ => {continue;}
        };

        let mut time: &String = &"".to_string();

        match worksheet_data.get((row_index, 2)) {
            Some(Data::String(time_data)) => time = time_data,
            _ => {}
        }

        log::trace!("{:?}", worksheet_data.get((row_index, column_index + 2)));
        let classroom = match worksheet_data.get((row_index, column_index + 2)) {
            Some(Data::String(classroom_numbers)) => {
                if classroom_numbers.contains("/") {
                    let classroom = classroom_numbers.split_once("/").ok_or_else(|| {
                        log::error!("Wtf");
                        "500 Internal Server Error"
                    })?;
                    if sheet_name.contains("не") {
                        classroom.0
                    } else {
                        classroom.1
                    }
                } else {
                    ""
                }
            }
            Some(Data::Int(classroom)) => &classroom.to_string(),
            Some(Data::Float(classroom)) => &format!("{:.0}", classroom),
            _ => {"Nodata"}
        };

        response.push_str(&format!(
            "{}: Время: <b>{}</b>, Аудитория: <b>{}</b> <blockquote>{}</blockquote> \n",
            lesson_index + 1,
            time,
            classroom,
            lesson
        ));
    }
    trace!("{:?}", worksheet_data.get((8, column_index)));
    Ok(response)
}
