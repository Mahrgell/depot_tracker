use std::rc::Rc;

use crate::instruments::{Instrument, MValue};

#[derive(Debug)]
pub struct Transaction {
    pub amount: i32,
    pub instrument: Rc<Instrument>,
    pub price: MValue,
    pub fees: MValue,
}
