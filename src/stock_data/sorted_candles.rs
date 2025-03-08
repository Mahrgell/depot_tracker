use super::CandleData;

#[derive(Debug, Default)]
pub struct SortedCandles {
    candles: Vec<CandleData>,
}

impl SortedCandles {
    pub fn add(&mut self, mut data: Vec<CandleData>) {
        self.candles.append(&mut data);
        self.candles.sort_by_key(|c| c.date);
        self.candles.dedup_by_key(|c| c.date);
    }

    pub fn candles(&self) -> &Vec<CandleData> {
        &self.candles
    }
}
