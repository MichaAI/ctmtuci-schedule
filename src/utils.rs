use chrono::Datelike;
use log::debug;
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
