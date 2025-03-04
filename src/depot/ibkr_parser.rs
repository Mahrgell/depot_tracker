use std::{
    fs::File,
    io::{self, Read},
    path::Path,
    rc::Rc,
};

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, TimeZone};
use chrono_tz::America::New_York;
use csv::ReaderBuilder;
use regex::Regex;

use crate::instruments::{Instrument, InstrumentList, OptionType, Stock, StockOption};

use super::{Event, StockSplit, Transaction};

#[derive(Debug, Default)]
pub struct IbkrParser {
    renames: Vec<(String, String)>,
    pub instruments: InstrumentList,
    pub events: Vec<Event>,
}

impl IbkrParser {
    pub fn read_renames<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut content = String::new();
        File::open(path)?.read_to_string(&mut content)?;
        for l in content.lines() {
            let names: Vec<_> = l.split("-->").collect();
            assert_eq!(names.len(), 2);
            self.renames.push((names[0].into(), names[1].into()));
        }
        Ok(())
    }

    pub fn parse<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;

        for line in reader.records() {
            let result = line?;
            let vals: Vec<_> = result.iter().collect();
            if vals.len() >= 11 && vals[0] == "Corporate Actions" && vals[3] == "USD" {
                self.parse_corporate_action(vals);
                continue;
            }

            if vals.len() < 16 || vals[0] != "Trades" || vals[1] != "Data" || vals[2] != "Order" {
                continue;
            }
            if vals[4] != "USD" {
                continue;
            }
            if self.parse_transaction(vals).is_err() {
                println!("Failed to parse: {:?}", result);
            }
        }

        Ok(())
    }

    fn parse_corporate_action(&mut self, vals: Vec<&str>) {
        if vals[7].parse::<f32>().unwrap() < 0. {
            // -> stock splits always occur twice, once with a positive value for stocks added and once with a ngeative amount for stocks taken out
            // -> ignore one line, as we only care about the factor
            return;
        }

        if let Some(spl) = self.parse_stock_split(vals[2], vals[5], vals[6]) {
            self.events.push(Event::Split(spl));
        }
    }

    fn parse_stock_split(
        &mut self,
        itype: &str,
        date: &str,
        description: &str,
    ) -> Option<StockSplit> {
        let mut date = parse_date_time(date);
        date -= Duration::seconds(1); // Hack: just to get ahead of eventual synchronous transactions...
        let pattern = r"^([a-zA-Z0-9]+)\([a-zA-Z0-9]+\) Split (\d+) for (\d+).*$";
        let re = Regex::new(pattern).unwrap();

        let caps = re.captures(description)?;
        let symbol = caps.get(1)?.as_str().to_string();
        let instrument = self.parse_instrument(itype, symbol).ok()?;
        let after: u32 = caps.get(2)?.as_str().parse().ok()?;
        let before: u32 = caps.get(3)?.as_str().parse().ok()?;
        let factor = after as f32 / before as f32;

        Some(StockSplit {
            instrument,
            factor,
            date,
        })
    }

    fn parse_transaction(&mut self, vals: Vec<&str>) -> Result<(), ()> {
        let date = parse_date_time(vals[6]);
        let instrument = self.parse_instrument(vals[3], vals[5].into())?;
        let amount = vals[7].replace(",", "").parse().unwrap();
        let price = vals[8].parse().unwrap();
        let fees = -vals[11].parse::<f32>().unwrap();
        let is_expiry = vals[15].contains("Ep");

        let tx = Transaction::new(date, amount, instrument, price, fees, is_expiry);
        self.events.push(Event::Transaction(tx));
        Ok(())
    }

    fn parse_instrument(&mut self, itype: &str, mut name: String) -> Result<Rc<Instrument>, ()> {
        if let Some((on, _)) = self.renames.iter().find(|(_, nn)| *nn == name) {
            name = on.into();
        }
        let instr = match itype {
            "Stocks" => Instrument::new(Stock::new(name)),
            "Equity and Index Options" => Instrument::new(self.parse_stock_option_name(&name)),
            _ => return Err(()),
        };
        Ok(self.instruments.add_or_get(instr))
    }

    fn parse_stock_option_name(&mut self, name: &str) -> StockOption {
        let parts: Vec<_> = name.split(' ').collect();
        assert_eq!(parts.len(), 4);
        let underlying = Instrument::new(Stock::new(parts[0].into()));
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
