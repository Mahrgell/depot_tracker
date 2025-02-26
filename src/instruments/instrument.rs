use super::{InstrumentSpec, Stock, StockOption};

#[derive(Debug)]
pub enum Instrument {
    Stock(Stock),
    StockOption(StockOption),
}

impl Instrument {
    fn to_spec(&self) -> Box<&dyn InstrumentSpec> {
        match self {
            Instrument::Stock(stock) => Box::new(stock),
            Instrument::StockOption(stock_option) => Box::new(stock_option),
        }
    }
}

impl InstrumentSpec for Instrument {
    fn as_instrument(self) -> Instrument {
        self
    }

    fn name(&self) -> &str {
        self.to_spec().name()
    }
}
