use std::{io, path::Path, rc::Rc};

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use chrono_tz::America::New_York;
use csv::ReaderBuilder;

use crate::instruments::{Instrument, InstrumentList, OptionType, Stock, StockOption};

use super::Transaction;

#[derive(Debug, Default)]
pub struct IbkrParser {
    pub instruments: InstrumentList,
    pub transactions: Vec<Rc<Transaction>>,
}

impl IbkrParser {
    pub fn parse<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;

        for result in reader.records() {
            let result = result?;
            let vals: Vec<_> = result.iter().collect();
            if vals.len() < 16 || vals[0] != "Trades" || vals[1] != "Data" || vals[2] != "Order" {
                continue;
            }
            if vals[4] != "USD" {
                continue;
            }
            self.parse_transaction(vals);
        }

        Ok(())
    }

    fn parse_transaction(&mut self, vals: Vec<&str>) {
        let date = parse_date_time(vals[6]);
        let instrument = self.parse_instrument(vals[3], vals[5]);
        let amount = vals[7].parse().unwrap();
        let price = vals[8].parse().unwrap();
        let fees = vals[11].parse().unwrap();

        let tx = Transaction::new(date, amount, instrument, price, fees);
        self.transactions.push(tx);
    }

    fn parse_instrument(&mut self, itype: &str, name: &str) -> Rc<Instrument> {
        let instr = match itype {
            "Stocks" => Instrument::new(Stock::new(name.into()), 0.),
            "Equity and Index Options" => Instrument::new(self.parse_stock_option_name(name), 0.),
            _ => unreachable!(),
        };
        self.instruments.add_or_get(instr)
    }

    fn parse_stock_option_name(&mut self, name: &str) -> StockOption {
        let parts: Vec<_> = name.split(' ').collect();
        assert_eq!(parts.len(), 4);
        let underlying = Instrument::new(Stock::new(parts[0].into()), 0.);
        let underlying = self.instruments.add_or_get(underlying);
        let strike = parts[2].parse().unwrap();
        let o_type = match parts[3] {
            "P" => OptionType::Put,
            "C" => OptionType::Call,
            _ => unreachable!(),
        };
        let expiry = NaiveDate::parse_from_str(parts[1], "%d%b%y").unwrap();
        StockOption::new(o_type, underlying, strike, 100, expiry)
    }
}

fn parse_date_time(dt: &str) -> DateTime<Local> {
    static FORMAT: &str = "%Y-%m-%d, %H:%M:%S";

    let naive_dt = NaiveDateTime::parse_from_str(dt, FORMAT).expect("Failed to parse datetime");

    let eastern_dt = New_York
        .from_local_datetime(&naive_dt)
        .single()
        .expect("Ambiguous or invalid datetime");

    eastern_dt.with_timezone(&Local)
}
