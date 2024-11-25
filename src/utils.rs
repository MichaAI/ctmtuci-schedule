use log::debug;

fn parse_date(date: &str) -> Result<chrono::NaiveDate, ()> {
    let date = date.to_lowercase();
    if date.is_empty() {
        return Err(());
    } else if date == "сегодня" || date == "today" || date == "now" {
        return Ok(chrono::Local::now().date_naive());
    } else if date == "завтра" || date == "tomorrow" {
        return Ok(chrono::Local::now().date_naive() + chrono::Duration::days(1));
    } else {
        let formats = [
            "%d.%m.%Y",
            "%d.%m",
            "%d.%m.%y",
            "%d/%m/%Y",
            "%d/%m",
            "%d/%m/%y",
            "%d-%m-%Y",
            "%d-%m",
            "%d-%m-%y",
            "%A"
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