use crate::instruments::{Instrument, MValue};
use chrono::{DateTime, Local};
use std::rc::Rc;

#[derive(Debug)]
pub struct Transaction {
    pub date: DateTime<Local>,
    pub amount: i32,
    pub instrument: Rc<Instrument>,
    pub price: MValue,
    pub fees: MValue,
}
