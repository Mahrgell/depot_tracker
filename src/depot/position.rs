use std::{cmp::Ordering, rc::Rc};

use crate::instruments::Instrument;

use super::{Trade, TransactionLink, TransactionT};

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

    pub fn apply_transaction<T: TransactionT>(&mut self, tx: T) -> Option<Trade> {
        assert!(self.instrument.eq(tx.instrument()));
        let tx_a = tx.amount();
        let trade = if tx_a * self.amount < 0 {
            let (open_txs, close_tx) = match tx_a.abs().cmp(&self.amount.abs()) {
                Ordering::Less => {
                    let mut rem = -tx_a;
                    let mut open_txs = Vec::new();
                    while rem != 0 {
                        if self.txs[0].amount().abs() > rem.abs() {
                            let (open_tx, rem_tx) = self.txs[0].split(rem);
                            open_txs.push(open_tx);
                            self.txs[0] = rem_tx;
                        } else {
                            rem -= self.txs[0].amount();
                            open_txs.push(self.txs.remove(0));
                        }
                    }
                    (open_txs, tx.as_link())
                }
                Ordering::Equal => (std::mem::take(&mut self.txs), tx.as_link()),
                Ordering::Greater => {
                    let open_txs = std::mem::take(&mut self.txs);
                    let (close_tx, rem_tx) = tx.as_link().split(self.amount);
                    self.txs.push(rem_tx);
                    (open_txs, close_tx)
                }
            };
            Some(Trade::new(open_txs, close_tx))
        } else {
            self.txs.push(tx.as_link());
            None
        };
        self.amount += tx_a;
        trade
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
