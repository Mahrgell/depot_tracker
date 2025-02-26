use crate::instruments::{InstrumentId, MValue};

#[derive(Debug)]
pub struct Transaction {
    pub amount: i32,
    pub instrument: InstrumentId,
    pub price: MValue,
    pub fees: MValue,
}
