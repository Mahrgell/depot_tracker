use std::{collections::HashMap, fs::File, io::Read, path::Path};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::stock_data::{sources::utils::limit_to_dates, CandleData};

use super::{DataSource, DataSourceError};

#[derive(Debug)]
pub struct AlphaVantage {
    apikey: String,
}

impl AlphaVantage {
    pub fn from_apikey_file<P: AsRef<Path>>(path: P) -> Result<AlphaVantage, std::io::Error> {
        let mut f = File::open(path)?;
        let mut apikey = String::new();
        f.read_to_string(&mut apikey)?;
        Ok(AlphaVantage { apikey })
    }

    fn get_query_params(&self, symbol: String) -> QueryParams {
        QueryParams {
            function: "TIME_SERIES_DAILY".into(),
            symbol,
            outputsize: "full".into(),
            apikey: self.apikey.clone(),
        }
    }
}

impl DataSource for AlphaVantage {
    fn get_data(
        &self,
        symbol: String,
        from: Option<chrono::NaiveDate>,
        to: Option<chrono::NaiveDate>,
    ) -> Result<Vec<CandleData>, DataSourceError> {
        // example link
        // https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&outputsize=full&apikey=demo
        let mut url = Url::parse("https://www.alphavantage.co/query").unwrap();
        let query_params = serde_urlencoded::to_string(self.get_query_params(symbol)).unwrap();
        url.set_query(Some(&query_params));
        dbg!(url.as_str());

        let mut data = match fetch_candle_data(url.as_str()) {
            Ok(data) => data,
            Err(err) => {
                dbg!(err);
                return Err(DataSourceError::FailedToParse);
            }
        };
        limit_to_dates(&mut data, from, to);
        Ok(data)
    }
}

#[derive(Serialize)]
struct QueryParams {
    function: String,
    symbol: String,
    outputsize: String,
    apikey: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(rename = "Time Series (Daily)")]
    time_series: HashMap<String, CandleEntry>,
}

#[derive(Debug, Deserialize)]
struct CandleEntry {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

pub fn fetch_candle_data(url: &str) -> Result<Vec<CandleData>, reqwest::Error> {
    let response = reqwest::blocking::get(url)?.json::<ApiResponse>()?;

    let mut candles = Vec::new();
    for (date_str, entry) in response.time_series {
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            if let (Ok(open), Ok(high), Ok(low), Ok(close), Ok(volume)) = (
                entry.open.parse::<f32>(),
                entry.high.parse::<f32>(),
                entry.low.parse::<f32>(),
                entry.close.parse::<f32>(),
                entry.volume.parse::<u32>(),
            ) {
                candles.push(CandleData {
                    date,
                    open,
                    high,
                    low,
                    close,
                    volume,
                });
            }
        }
    }
    candles.sort_by_key(|cd| cd.date);

    Ok(candles)
}
