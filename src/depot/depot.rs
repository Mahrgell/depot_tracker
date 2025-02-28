use std::rc::Rc;

use crate::instruments::{InstrumentList, MValue};

use super::{Position, Transaction, TransactionT};

#[derive(Debug, Default)]
pub struct Depot {
    positions: Vec<Position>,
    cash: MValue,
    _instruments: InstrumentList,
    transactions: Vec<Rc<Transaction>>,
}

impl Depot {
    pub fn _instruments(&self) -> &InstrumentList {
        &self._instruments
    }

    pub fn positions(&self) -> &Vec<Position> {
        &self.positions
    }

    pub fn _deposit(&mut self, amount: MValue) {
        assert!(amount > 0.);
        self.cash += amount;
    }

    pub fn add_transactions(&mut self, mut txs: Vec<Rc<Transaction>>) {
        if txs.is_empty() {
            return;
        }
        let first_new_tx = txs.iter().map(|tx| tx.date()).min().unwrap();
        if self
            .transactions
            .last()
            .map_or(false, |last_tx| last_tx.date() > first_new_tx)
        {
            txs.append(&mut self.transactions);
        }
        txs.sort_by_key(|tx| tx.date());
        for tx in &txs {
            self.apply_transaction(tx.clone());
        }
        self.transactions.append(&mut txs);
    }

    fn apply_transaction(&mut self, tx: Rc<Transaction>) {
        self.cash -= tx.total_cost();
        if let Some(i) = self
            .positions
            .iter()
            .position(|pos| Rc::ptr_eq(pos.instrument(), tx.instrument()))
        {
            let pos = &mut self.positions[i];
            pos.apply_transaction(tx);
            if pos.is_empty() {
                self.positions.remove(i);
            }
        } else {
            self.positions.push(tx.into());
        }
    }
}
