use super::{instrument_spec::InstrumentSpec, InstrumentWrapped};

#[derive(Debug, PartialEq)]
pub struct Stock {
    symbol: String,
}

impl Stock {
    pub fn new(symbol: String) -> Self {
        Stock { symbol }
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }
}

impl InstrumentSpec for Stock {
    fn as_wrapped(self) -> InstrumentWrapped {
        InstrumentWrapped::Stock(self)
    }

    fn name(&self) -> &str {
        &self.symbol
    }

    fn factor(&self) -> u32 {
        1
    }

    fn matches_symbol(&self, symbol: &str, _include_underlying: bool) -> bool {
        self.symbol == symbol
    }
}
