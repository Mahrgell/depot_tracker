use std::{
    rc::Rc,
    sync::{RwLock, RwLockReadGuard},
};

use crate::{
    properties::{FormattedProperty, InstrumentName, Price, Property},
    stock_data::sources::{DataSource, DataSourceError, LocalFile},
};

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
    pub fn new<I: InstrumentSpec>(instr: I) -> Rc<Self> {
        Rc::new(Self {
            instr: instr.as_wrapped(),
            data: RwLock::new(InstrumentData::default()),
        })
    }

    pub fn eq(self: &Rc<Self>, other: &Rc<Self>) -> bool {
        Rc::ptr_eq(self, other)
    }

    pub fn info(&self) -> &InstrumentWrapped {
        &self.instr
    }

    pub fn data<'a>(&'a self) -> RwLockReadGuard<'a, InstrumentData> {
        self.data.read().unwrap()
    }

    pub fn price(&self) -> Option<MValue> {
        self.data.read().unwrap().price(None)
    }

    pub fn update_data_with(
        &self,
        ds: &impl DataSource,
    ) -> Result<(usize, usize), DataSourceError> {
        let new_candles = ds.get_data(self.instr.name().into(), None, None)?;
        let new_candles_read = new_candles.len();
        let mut data = self.data.write().unwrap();
        let data_points_before = data.nb_data_points();
        data.add_data(new_candles);
        Ok((new_candles_read, data.nb_data_points() - data_points_before))
    }

    pub fn save_data_local(&self, lf: &LocalFile) -> Result<(), ()> {
        let data = self.data.read().unwrap();
        let candles = data.get_raw();
        if candles.is_empty() {
            return Err(());
        }
        lf.save_local(self.instr.name().into(), candles)
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

    fn get_related_instruments(&self, res: &mut Vec<Rc<Instrument>>) {
        self.to_spec().get_related_instruments(res)
    }

    fn matches_symbol(&self, symbol: &str, include_underlying: bool) -> bool {
        self.to_spec().matches_symbol(symbol, include_underlying)
    }
}

impl<T: FormattedProperty<Instrument>> FormattedProperty<Rc<Instrument>> for T {
    fn header(&self) -> String {
        <T as FormattedProperty<Instrument>>::header(self)
    }

    fn format_data(&mut self, t: &Rc<Instrument>) -> String {
        self.format_data(t.as_ref())
    }
}

impl Property<Price> for Instrument {
    fn get(&self, _: &Price) -> Option<MValue> {
        self.price()
    }
}

impl Property<InstrumentName> for Instrument {
    fn get(&self, _: &InstrumentName) -> String {
        self.info().name().into()
    }
}
