use std::{rc::Rc, sync::RwLock};

use super::{InstrumentData, InstrumentSpec, MValue, Stock, StockOption};

#[derive(Debug)]
pub enum InstrumentWrapped {
    Stock(Stock),
    StockOption(StockOption),
}

#[derive(Debug)]
pub struct Instrument {
    instr: InstrumentWrapped,
    data: RwLock<InstrumentData>,
}

impl Instrument {
    pub fn new<I: InstrumentSpec>(instr: I, price: MValue) -> Rc<Self> {
        Rc::new(Self {
            instr: instr.as_wrapped(),
            data: RwLock::new(InstrumentData { price }),
        })
    }

    pub fn price(&self) -> MValue {
        self.data.read().unwrap().price
    }
}

impl InstrumentWrapped {
    fn to_spec(&self) -> Box<&dyn InstrumentSpec> {
        match self {
            InstrumentWrapped::Stock(stock) => Box::new(stock),
            InstrumentWrapped::StockOption(stock_option) => Box::new(stock_option),
        }
    }
}

impl InstrumentSpec for InstrumentWrapped {
    fn as_wrapped(self) -> InstrumentWrapped {
        self
    }

    fn name(&self) -> &str {
        self.to_spec().name()
    }
}
