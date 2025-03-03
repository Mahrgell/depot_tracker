use chrono::NaiveDate;

use crate::stock_data::CandleData;

pub fn limit_to_dates(data: &mut Vec<CandleData>, from: Option<NaiveDate>, to: Option<NaiveDate>) {
    if let Some(to) = to {
        data.retain(|cd| cd.date < to);
    }
    if let Some(from) = from {
        data.retain(|cd| from <= cd.date);
    }
}
