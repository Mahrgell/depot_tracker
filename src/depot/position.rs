use crate::instruments::InstrumentId;

use super::Transaction;

#[derive(Debug)]
pub struct Position {
    pub amount: i32,
    pub instrument: InstrumentId,
}

impl Position {
    pub fn from_transaction(tx: &Transaction) -> Self {
        Position {
            amount: tx.amount,
            instrument: tx.instrument,
        }
    }
}
