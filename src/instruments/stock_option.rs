use super::{Instrument, InstrumentId, InstrumentSpec, MValue};

#[derive(Copy, Clone, Debug)]
pub enum OptionType {
    Put,
    Call,
}

#[derive(Debug)]
pub struct StockOption {
    name: String,
    o_type: OptionType,
    underlying: InstrumentId,
    strike: MValue,
    factor: u32,
}

impl InstrumentSpec for StockOption {
    fn as_instrument(self) -> Instrument {
        Instrument::StockOption(self)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
