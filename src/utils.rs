use chrono::Datelike;
use log::debug;
use chrono::{NaiveDate, Duration};

pub fn parse_date(date: &str) -> Result<chrono::NaiveDate, ()> {
    let date = date.to_lowercase();
    if date.is_empty() {
        return Err(());
    } else if date == "сегодня" || date == "today" || date == "now" {
        return Ok(chrono::Local::now().date_naive());
    } else if date == "завтра" || date == "tomorrow" {
        return Ok(chrono::Local::now().date_naive() + chrono::Duration::days(1));
    } else if date == "вчера" || date == "yesterday" {
        return Ok(chrono::Local::now().date_naive() - chrono::Duration::days(1));
    } else if date == "послезавтра" || date == "after tomorrow" {
        return Ok(chrono::Local::now().date_naive() + chrono::Duration::days(2));
    } else if date == "позовчера" || date == "before yesterday" {
        return Ok(chrono::Local::now().date_naive() - chrono::Duration::days(2));
    } else if date == "понедельник" || date == "monday" {
        return Ok(chrono::Local::now().date_naive()
            - chrono::Duration::days(
                chrono::Local::now()
                    .date_naive()
                    .weekday()
                    .num_days_from_monday() as i64,
            ));
    } else if date == "вторник" || date == "tuesday" {
        return Ok(chrono::Local::now().date_naive()
            - chrono::Duration::days(
                chrono::Local::now()
                    .date_naive()
                    .weekday()
                    .num_days_from_monday() as i64
                    - 1,
            ));
    } else if date == "среда" || date == "wednesday" {
        return Ok(chrono::Local::now().date_naive()
            - chrono::Duration::days(
                chrono::Local::now()
                    .date_naive()
                    .weekday()
                    .num_days_from_monday() as i64
                    - 2,
            ));
    } else if date == "четверг" || date == "thursday" {
        return Ok(chrono::Local::now().date_naive()
            - chrono::Duration::days(
                chrono::Local::now()
                    .date_naive()
                    .weekday()
                    .num_days_from_monday() as i64
                    - 3,
            ));
    } else if date == "пятница" || date == "friday" {
        return Ok(chrono::Local::now().date_naive()
            - chrono::Duration::days(
                chrono::Local::now()
                    .date_naive()
                    .weekday()
                    .num_days_from_monday() as i64
                    - 4,
            ));
    } else if date == "суббота" || date == "saturday" {
        return Ok(chrono::Local::now().date_naive()
            - chrono::Duration::days(
                chrono::Local::now()
                    .date_naive()
                    .weekday()
                    .num_days_from_monday() as i64
                    - 5,
            ));
    } else if date == "воскресенье" || date == "sunday" {
        return Ok(chrono::Local::now().date_naive()
            - chrono::Duration::days(
                chrono::Local::now()
                    .date_naive()
                    .weekday()
                    .num_days_from_monday() as i64
                    - 6,
            ));
    } else {
        let formats = [
            "%d.%m.%Y", "%d.%m", "%d.%m.%y", "%d/%m/%Y", "%d/%m", "%d/%m/%y", "%d-%m-%Y", "%d-%m",
            "%d-%m-%y", "%A",
        ];

        for format in formats.iter() {
            if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(&date, format) {
                debug!("Date parsed: {}", parsed_date);
                return Ok(parsed_date);
            }
        }
        Err(())
    }
}

pub fn get_calendar(date: Option<chrono::NaiveDate>) -> (Vec<Vec<Option<chrono::NaiveDate>>>, String) {
    // Получение текущей даты
    let today = match date {
        Some(date) => date.into(),
        None => chrono::Local::now().naive_local(),
    };

    let year = today.year();
    let month = today.month();

    // Первый день текущего месяца
    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

    // Первый день следующего месяца
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let first_day_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();

    // Последний день текущего месяца
    let last_day_of_month = first_day_next_month - Duration::days(1);
    let num_days_in_month = last_day_of_month.day();

    // Определяем день недели первого числа месяца (0 = Понедельник)
    let first_weekday = first_day_of_month.weekday().num_days_from_monday();

    // Инициализируем календарь
    let mut calendar: Vec<Vec<Option<NaiveDate>>> = Vec::new();
    let mut week: Vec<Option<NaiveDate>> = Vec::new();

    // Заполняем первую неделю пустыми значениями до первого дня
    for _ in 0..first_weekday {
        week.push(None);
    }

    // Заполняем даты месяца
    for day in 1..=num_days_in_month {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        week.push(Some(date));

        if week.len() == 7 {
            calendar.push(week);
            week = Vec::new();
        }
    }

    // Если остались дни в неделе, дополняем её пустыми значениями и добавляем в календарь
    if !week.is_empty() {
        while week.len() < 7 {
            week.push(None);
        }
        calendar.push(week);
    }

    (calendar, today.format("%B %Y").to_string()) // Возвращаем calendar
}

#[test]
fn test_get_calendar() {
    assert_eq!(get_calendar(None).0.len(), 5);
    

    let calendar = get_calendar(None);

    // Выводим календарь для проверки
    for week in calendar.0 {
        for day in week {
            match day {
                Some(date) => print!("{:2} ", date.day()),
                None => print!("   "),
            }
        }
        println!();
    }
}