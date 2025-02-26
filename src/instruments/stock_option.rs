use std::rc::Rc;

use super::{Instrument, InstrumentSpec, InstrumentWrapped, MValue};

#[derive(Copy, Clone, Debug)]
pub enum OptionType {
    Put,
    Call,
}

#[derive(Debug)]
pub struct StockOption {
    name: String,
    o_type: OptionType,
    underlying: Rc<Instrument>,
    strike: MValue,
    factor: u32,
}

impl InstrumentSpec for StockOption {
    fn as_wrapped(self) -> InstrumentWrapped {
        InstrumentWrapped::StockOption(self)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
