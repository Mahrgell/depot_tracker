use std::rc::Rc;

use crate::instruments::Instrument;

use super::{Transaction, TransactionT};

#[derive(Debug)]
pub struct Position {
    pub amount: i32,
    pub instrument: Rc<Instrument>,
}

impl Position {
    pub fn from_transaction(tx: &Transaction) -> Self {
        Position {
            amount: tx.amount(),
            instrument: tx.instrument().clone(),
        }
    }
}
