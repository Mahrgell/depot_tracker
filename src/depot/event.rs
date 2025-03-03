use std::rc::Rc;

use chrono::{DateTime, Local};

use crate::instruments::{HasInstrument, Instrument};

use super::{StockSplit, Transaction, TransactionT};

#[derive(Clone, Debug)]
pub enum Event {
    Transaction(Rc<Transaction>),
    Split(StockSplit),
}

impl Event {
    pub fn date(&self) -> DateTime<Local> {
        match self {
            Event::Transaction(tx) => tx.date(),
            Event::Split(spl) => spl.date,
        }
    }
}

impl HasInstrument for Event {
    fn instrument(&self) -> &Rc<Instrument> {
        match self {
            Event::Transaction(tx) => tx.instrument(),
            Event::Split(spl) => spl.instrument(),
        }
    }
}
