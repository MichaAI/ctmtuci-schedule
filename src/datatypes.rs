use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageMetadata {
    pub date: NaiveDate,
    pub group: String,
}
