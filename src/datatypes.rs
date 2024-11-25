use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageMetadata {
    #[serde(with = "chrono::NaiveDate")]
    pub date: NaiveDate,
    pub group: String,
}
