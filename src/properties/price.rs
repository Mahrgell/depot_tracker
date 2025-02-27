use crate::{depot::Position, instruments::Instrument};

use super::Property;

pub struct Price {}

impl Price {
    pub fn new<T>() -> Box<dyn Property<T>>
    where
        Self: Property<T>,
    {
        Box::new(Self {})
    }
}

impl Property<Position> for Price {
    fn header(&self) -> String {
        "Price".into()
    }

    fn format_data(&self, t: &Position) -> String {
        self.format_data(&t.instrument)
    }
}

impl Property<Instrument> for Price {
    fn header(&self) -> String {
        "Price".into()
    }

    fn format_data(&self, t: &Instrument) -> String {
        format!("{:.2}", t.price())
    }
}
