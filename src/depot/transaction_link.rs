use std::rc::Rc;

use chrono::{DateTime, Local};

use crate::instruments::{HasInstrument, Instrument, MValue};

use super::{Transaction, TransactionT};

#[derive(Debug)]
pub struct TransactionLink {
    tx: Rc<Transaction>,
    partial: Option<f32>,
}

impl TransactionLink {
    pub fn split(&self, amount: f32) -> (TransactionLink, TransactionLink) {
        let old_a = self.amount();
        assert!((old_a > amount && amount > 0.) || (old_a < amount && amount < 0.));
        (
            TransactionLink {
                tx: self.tx.clone(),
                partial: Some(amount),
            },
            TransactionLink {
                tx: self.tx.clone(),
                partial: Some(old_a - amount),
            },
        )
    }
}

impl From<Rc<Transaction>> for TransactionLink {
    fn from(tx: Rc<Transaction>) -> Self {
        TransactionLink { tx, partial: None }
    }
}

impl HasInstrument for TransactionLink {
    fn instrument(&self) -> &Rc<Instrument> {
        &self.tx.instrument()
    }
}

impl TransactionT for TransactionLink {
    fn date(&self) -> DateTime<Local> {
        self.tx.date()
    }

    fn amount(&self) -> f32 {
        match self.partial {
            Some(p) => p,
            None => self.tx.amount(),
        }
    }

    fn price(&self) -> MValue {
        self.tx.price()
    }

    fn fees(&self) -> MValue {
        match self.partial {
            Some(p) => p / self.tx.amount() * self.tx.fees(),
            None => self.tx.fees(),
        }
    }

    fn is_expiry(&self) -> bool {
        self.tx.is_expiry()
    }

    fn as_link(self) -> TransactionLink {
        self
    }
}
