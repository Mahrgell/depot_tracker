use std::rc::Rc;

use chrono::{DateTime, Local};

use crate::instruments::{HasInstrument, Instrument};

#[derive(Clone, Debug)]
pub struct StockSplit {
    pub instrument: Rc<Instrument>,
    pub factor: f32,
    pub date: DateTime<Local>,
}

impl HasInstrument for StockSplit {
    fn instrument(&self) -> &Rc<Instrument> {
        &self.instrument
    }
}
