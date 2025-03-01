use crate::instruments::{HasInstrument, Instrument, InstrumentSpec, MValue};
use chrono::{DateTime, Local};
use std::rc::Rc;

use super::TransactionLink;

pub trait TransactionT: HasInstrument {
    fn date(&self) -> DateTime<Local>;
    fn amount(&self) -> i32;
    fn price(&self) -> MValue;
    fn fees(&self) -> MValue;
    fn as_link(self) -> TransactionLink;

    fn total_cost(&self) -> MValue {
        self.amount() as f32 * self.price() * self.instrument().info().factor() as f32 + self.fees()
    }
}

#[derive(Debug)]
pub struct Transaction {
    date: DateTime<Local>,
    amount: i32,
    instrument: Rc<Instrument>,
    price: MValue,
    fees: MValue,
}

impl Transaction {
    pub fn new(
        date: DateTime<Local>,
        amount: i32,
        instrument: Rc<Instrument>,
        price: MValue,
        fees: MValue,
    ) -> Rc<Self> {
        assert_ne!(amount, 0);
        Rc::new(Transaction {
            date,
            amount,
            instrument,
            price,
            fees,
        })
    }
}

impl TransactionT for Rc<Transaction> {
    fn date(&self) -> DateTime<Local> {
        self.date
    }

    fn amount(&self) -> i32 {
        self.amount
    }

    fn price(&self) -> MValue {
        self.price
    }

    fn fees(&self) -> MValue {
        self.fees
    }

    fn as_link(self) -> TransactionLink {
        self.into()
    }
}

impl HasInstrument for Rc<Transaction> {
    fn instrument(&self) -> &Rc<Instrument> {
        &self.instrument
    }
}
