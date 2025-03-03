use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use chrono::NaiveDate;

use crate::stock_data::CandleData;

use super::{utils::limit_to_dates, DataSource, DataSourceError};

const FILEENDING: &str = ".dtcd";

pub struct LocalFile {
    dir: PathBuf,
}

impl LocalFile {
    pub fn new(dir: PathBuf) -> Self {
        LocalFile { dir }
    }

    pub fn save_local(&self, symbol: String, data: &Vec<CandleData>) -> Result<(), ()> {
        let path = self.get_path(symbol);
        let file = File::create(path).map_err(|_| ())?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, data).map_err(|_| ())?;
        Ok(())
    }

    fn get_path(&self, symbol: String) -> PathBuf {
        self.dir.join(symbol + FILEENDING)
    }
}

impl DataSource for LocalFile {
    fn get_data(
        &self,
        symbol: String,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<CandleData>, DataSourceError> {
        let path = self.get_path(symbol);
        let file = File::open(path).map_err(|_| DataSourceError::NotAvailable)?;
        let reader = BufReader::new(file);
        let mut data =
            serde_json::from_reader(reader).map_err(|_| DataSourceError::FailedToParse)?;
        limit_to_dates(&mut data, from, to);
        Ok(data)
    }
}
