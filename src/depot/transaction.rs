use crate::instruments::{HasInstrument, Instrument, InstrumentSpec, MValue};
use chrono::{DateTime, Local};
use std::rc::Rc;

use super::TransactionLink;

pub trait TransactionT: HasInstrument + std::fmt::Debug {
    fn date(&self) -> DateTime<Local>;
    fn amount(&self) -> f32;
    fn price(&self) -> MValue;
    fn fees(&self) -> MValue;
    fn is_expiry(&self) -> bool;
    fn as_link(self) -> TransactionLink;

    fn total_cost(&self) -> MValue {
        self.amount() * self.price() * self.instrument().info().factor() as f32 + self.fees()
    }
}

#[derive(Debug)]
pub struct Transaction {
    date: DateTime<Local>,
    amount: f32,
    instrument: Rc<Instrument>,
    price: MValue,
    fees: MValue,
    is_expiry: bool,
}

impl Transaction {
    pub fn new(
        date: DateTime<Local>,
        amount: f32,
        instrument: Rc<Instrument>,
        price: MValue,
        fees: MValue,
        is_expiry: bool,
    ) -> Rc<Self> {
        assert_ne!(amount, 0.);
        Rc::new(Transaction {
            date,
            amount,
            instrument,
            price,
            fees,
            is_expiry,
        })
    }
}

impl TransactionT for Rc<Transaction> {
    fn date(&self) -> DateTime<Local> {
        self.date
    }

    fn amount(&self) -> f32 {
        self.amount
    }

    fn price(&self) -> MValue {
        self.price
    }

    fn fees(&self) -> MValue {
        self.fees
    }

    fn is_expiry(&self) -> bool {
        self.is_expiry
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
