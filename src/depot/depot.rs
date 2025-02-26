use std::rc::Rc;

use crate::instruments::{Instrument, MValue};

use super::{Position, Transaction};

#[derive(Debug, Default)]
pub struct Depot {
    positions: Vec<Position>,
    cash: MValue,
    _instruments: Vec<Instrument>,
}

impl Depot {
    pub fn _instruments(&self) -> &Vec<Instrument> {
        &self._instruments
    }

    pub fn deposit(&mut self, amount: MValue) {
        assert!(amount > 0.);
        self.cash += amount;
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) {
        if let Some(i) = self
            .positions
            .iter()
            .position(|pos| Rc::ptr_eq(&pos.instrument, &tx.instrument))
        {
            let pos = &mut self.positions[i];
            if pos.amount != -tx.amount {
                pos.amount += tx.amount;
            } else {
                self.positions.remove(i);
            }
        } else {
            self.positions.push(Position::from_transaction(&tx));
        }
        self.cash -= tx.amount as f32 * tx.price + tx.fees;
    }
}
