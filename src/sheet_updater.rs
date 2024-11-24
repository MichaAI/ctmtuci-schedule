use std::{collections::HashMap, io::Cursor, sync::{Arc, LazyLock}};

use calamine::{Data, Reader};
use chrono::Datelike;

use crate::{SHEET, URI};

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());
pub static GROOPS: LazyLock<Arc<tokio::sync::RwLock<HashMap<String, (usize, usize)>>>> = LazyLock::new(|| Arc::new(tokio::sync::RwLock::new(HashMap::new())));

async fn update() -> Result<(), Box<dyn std::error::Error>> {
    // Скачать файл асинхронноS
    let response = CLIENT.get(URI).send().await?;
    let bytes = response.bytes().await?;

    // Создать курсор для работы с данными в памяти
    let cursor = Cursor::new(bytes);

    // Открыть файл XLSX из курсора
    let workbook = calamine::open_workbook_auto_from_rs(cursor)?;
    let mut sheet = SHEET.lock().await;
    let sheet = sheet.as_mut();
    *sheet = Some(workbook);

    let mut groops = GROOPS.write().await;
    groops.clear();

    if let Some(sheet) = &mut *sheet {
        let today = chrono::Local::now().date_naive();
        let weekday = today.weekday();
        let monday = today - chrono::Duration::days(weekday.num_days_from_monday() as i64);
        let formated_maonday = monday.format("%d.%m").to_string();
        let mut name = sheet.sheet_names();
        let name = name.iter_mut()
        .filter(|s| s.starts_with(formated_maonday.as_str()))
        .next().unwrap();
        
        let list = sheet.worksheet_range(name).unwrap();
        list.range((4, 0), (4, 290)).cells().filter(|cell| {
            match cell.2 {
                Data::String(_) => true,
                _ => false
            }
        }).map(|cell| {
            if let Data::String(s) = cell.2 {
                return (cell.0, cell.1, s.as_str());
            }
            return (0, 0, "");
        }).for_each(|s| {
            groops.insert(s.2.to_lowercase().to_string(), (s.0, s.1));
        });
    }

    Ok(())
}

pub async fn start_update() -> Result<(), Box<dyn std::error::Error>> {
    let res = update().await;
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    }

    tokio::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));

        loop {
            interval.tick().await;
            let res = update().await;

            match res {
                Ok(_) => {}
                Err(e) => println!("Error: {}", e),
            }
        }
    });

    Ok(())
}
