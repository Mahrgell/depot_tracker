use chrono::NaiveDate;

use crate::stock_data::CandleData;

pub enum DataSourceError {
    FailedToParse,
    TemporarilyUnavailable,
    NotAvailable,
}

pub trait DataSource {
    fn get_data(
        &self,
        symbol: String,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<CandleData>, DataSourceError>;
}
