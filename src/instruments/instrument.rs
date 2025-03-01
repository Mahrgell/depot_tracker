use std::{rc::Rc, sync::RwLock};

use crate::properties::{FormattedProperty, Name, Price, Property};

use super::{InstrumentData, InstrumentSpec, MValue, Stock, StockOption};

#[derive(Debug, PartialEq)]
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

    pub fn eq(self: &Rc<Self>, other: &Rc<Self>) -> bool {
        Rc::ptr_eq(self, other)
    }

    pub fn info(&self) -> &InstrumentWrapped {
        &self.instr
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

impl PartialEq for Instrument {
    fn eq(&self, other: &Self) -> bool {
        self.instr == other.instr
    }
}

impl InstrumentSpec for InstrumentWrapped {
    fn as_wrapped(self) -> InstrumentWrapped {
        self
    }

    fn name(&self) -> &str {
        self.to_spec().name()
    }

    fn factor(&self) -> u32 {
        self.to_spec().factor()
    }
}

impl<T: FormattedProperty<Instrument>> FormattedProperty<Rc<Instrument>> for T {
    fn header(&self) -> String {
        <T as FormattedProperty<Instrument>>::header(self)
    }

    fn format_data(&self, t: &Rc<Instrument>) -> String {
        self.format_data(t.as_ref())
    }
}

impl Property<Price> for Instrument {
    fn get(&self, _: &Price) -> MValue {
        self.price()
    }
}

impl Property<Name> for Instrument {
    fn get(&self, _: &Name) -> String {
        self.info().name().into()
    }
}
