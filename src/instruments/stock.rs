use super::{instrument_spec::InstrumentSpec, Instrument};

#[derive(Debug)]
pub struct Stock {
    symbol: String,
}

impl Stock {
    pub fn new(symbol: String) -> Self {
        Stock { symbol }
    }
}

impl InstrumentSpec for Stock {
    fn as_instrument(self) -> Instrument {
        Instrument::Stock(self)
    }

    fn name(&self) -> &str {
        &self.symbol
    }
}
