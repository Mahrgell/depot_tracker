use std::rc::Rc;

use crate::{
    depot::TransactionT,
    instruments::{Instrument, MValue},
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

    pub fn instrument(&self) -> &Rc<Instrument> {
        self.close_tx.instrument()
    }

    pub fn profit(&self) -> MValue {
        -self
            .open_txs
            .iter()
            .fold(self.close_tx.total_cost(), |a, tx| a + tx.total_cost())
    }
}
