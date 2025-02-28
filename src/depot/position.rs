use std::rc::Rc;

use crate::instruments::Instrument;

use super::{TransactionLink, TransactionT};

#[derive(Debug)]
pub struct Position {
    amount: i32,
    instrument: Rc<Instrument>,
    txs: Vec<TransactionLink>,
}

impl Position {
    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn instrument(&self) -> &Rc<Instrument> {
        &self.instrument
    }

    pub fn is_empty(&self) -> bool {
        self.amount == 0
    }

    pub fn apply_transaction<T: TransactionT>(&mut self, tx: T) {
        assert!(Rc::ptr_eq(&self.instrument, &tx.instrument()));
        self.amount += tx.amount();
        self.txs.push(tx.as_link());
    }
}

impl<T: TransactionT> From<T> for Position {
    fn from(tx: T) -> Self {
        Position {
            amount: tx.amount(),
            instrument: tx.instrument().clone(),
            txs: vec![tx.as_link()],
        }
    }
}
