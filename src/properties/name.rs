use crate::{
    depot::Position,
    instruments::{Instrument, InstrumentSpec},
};

use super::Property;

pub struct Name {}

impl Name {
    pub fn new<T>() -> Box<dyn Property<T>>
    where
        Self: Property<T>,
    {
        Box::new(Self {})
    }
}

impl Property<Position> for Name {
    fn header(&self) -> String {
        "Instrument".into()
    }

    fn format_data(&self, t: &Position) -> String {
        self.format_data(t.instrument())
    }
}

impl Property<Instrument> for Name {
    fn header(&self) -> String {
        "Instrument".into()
    }

    fn format_data(&self, t: &Instrument) -> String {
        t.info().name().into()
    }
}
