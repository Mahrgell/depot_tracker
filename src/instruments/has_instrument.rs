use std::rc::Rc;

use crate::properties::{InstrumentName, Property};

use super::Instrument;

pub trait HasInstrument {
    fn instrument(&self) -> &Rc<Instrument>;
}

impl<T: HasInstrument> Property<InstrumentName> for T {
    fn get(&self, i: &InstrumentName) -> String {
        self.instrument().get(i)
    }
}
