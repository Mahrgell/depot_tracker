use super::CandleData;

#[derive(Debug, Default)]
pub struct SortedCandles {
    candles: Vec<CandleData>,
}

impl SortedCandles {
    pub fn new(mut candles: Vec<CandleData>) -> SortedCandles {
        candles.sort_by_key(|c| c.date);
        candles.dedup_by_key(|c| c.date);
        SortedCandles { candles }
    }

    pub fn candles(&self) -> &Vec<CandleData> {
        &self.candles
    }
}
