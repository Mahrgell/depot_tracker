use serde::{Deserialize, Serialize};

use super::CandleData;

#[derive(Debug, Serialize, Deserialize)]
pub struct StockData {
    data: Vec<CandleData>,
}
