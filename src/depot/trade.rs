use std::rc::Rc;

use chrono::NaiveDate;

use crate::{
    depot::TransactionT,
    instruments::{HasInstrument, Instrument, MValue},
    properties::{CloseDate, OpenDate, Profit, Property},
};

use super::TransactionLink;

#[derive(Debug)]
pub struct Trade {
    open_txs: Vec<TransactionLink>,
    close_tx: TransactionLink,
}

impl Trade {
    pub fn new(open_txs: Vec<TransactionLink>, close_tx: TransactionLink) -> Self {
        assert!(open_txs
            .iter()
            .all(|tx| close_tx.instrument().eq(tx.instrument())));
        Trade { open_txs, close_tx }
    }

    pub fn profit(&self) -> MValue {
        -self
            .open_txs
            .iter()
            .fold(self.close_tx.total_cost(), |a, tx| a + tx.total_cost())
    }
}

impl HasInstrument for Trade {
    fn instrument(&self) -> &Rc<Instrument> {
        &self.close_tx.instrument()
    }
}

impl Property<OpenDate> for Trade {
    fn get(&self, _: &OpenDate) -> (NaiveDate, Option<NaiveDate>) {
        let t1 = self.open_txs.first().unwrap().date().date_naive();
        let t2 = self.open_txs.last().unwrap().date().date_naive();
        let t2 = if t1 == t2 { None } else { Some(t2) };
        (t1, t2)
    }
}

impl Property<CloseDate> for Trade {
    fn get(&self, _: &CloseDate) -> NaiveDate {
        self.close_tx.date().date_naive()
    }
}

impl Property<Profit> for Trade {
    fn get(&self, _: &Profit) -> MValue {
        self.profit()
    }
}
