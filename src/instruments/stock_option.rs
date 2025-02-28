use std::rc::Rc;

use chrono::NaiveDate;

use super::{Instrument, InstrumentSpec, InstrumentWrapped, MValue};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OptionType {
    Put,
    Call,
}

#[derive(Debug, PartialEq)]
pub struct StockOption {
    name: String,
    o_type: OptionType,
    underlying: Rc<Instrument>,
    strike: MValue,
    factor: u32,
    expiry: NaiveDate,
}

impl StockOption {
    pub fn new(
        o_type: OptionType,
        underlying: Rc<Instrument>,
        strike: MValue,
        factor: u32,
        expiry: NaiveDate,
    ) -> Self {
        if let InstrumentWrapped::Stock(s) = underlying.info() {
            let name = format!(
                "{:?} {} {} - {}",
                o_type,
                s.symbol(),
                strike,
                expiry.format("%d.%m.%y")
            );
            Self {
                name,
                o_type,
                underlying,
                strike,
                factor,
                expiry,
            }
        } else {
            panic!("Stock option not based on stock")
        }
    }
}

impl InstrumentSpec for StockOption {
    fn as_wrapped(self) -> InstrumentWrapped {
        InstrumentWrapped::StockOption(self)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn factor(&self) -> u32 {
        self.factor
    }
}
