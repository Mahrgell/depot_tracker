use chrono::NaiveDate;

use crate::stock_data::SortedCandles;

#[derive(Debug, Default)]
pub struct InstrumentData {
    candles: SortedCandles,
}

impl InstrumentData {
    pub fn price(&self, date: Option<NaiveDate>) -> Option<f32> {
        match date {
            Some(date) => {
                match self
                    .candles
                    .candles()
                    .binary_search_by_key(&date, |c| c.date)
                {
                    Ok(idx) => Some(self.candles.candles()[idx].close),
                    Err(0) => None,
                    Err(idx) => Some(self.candles.candles()[idx - 1].close),
                }
            }
            None => self.candles.candles().last().map(|c| c.close),
        }
    }
}
