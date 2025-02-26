use std::ops::Index;

use super::{instrument_spec::InstrumentSpec, Instrument, InstrumentId};

#[derive(Debug, Default)]
pub struct InstrumentList {
    list: Vec<Instrument>,
}

impl InstrumentList {
    pub fn add<I: InstrumentSpec>(&mut self, instr: I) -> InstrumentId {
        self.list.push(instr.as_instrument());
        self.list.len() - 1
    }
}

impl Index<InstrumentId> for InstrumentList {
    type Output = Instrument;

    fn index(&self, index: InstrumentId) -> &Self::Output {
        &self.list[index]
    }
}
