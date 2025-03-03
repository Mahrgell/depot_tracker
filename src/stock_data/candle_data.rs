use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CandleData {
    pub date: NaiveDate,
    pub open: f32,
    pub close: f32,
    pub low: f32,
    pub high: f32,
    pub volume: u32,
}
