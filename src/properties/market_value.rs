use crate::{depot::Position, instruments::InstrumentSpec};

use super::Property;

pub struct MarketValue {}

impl MarketValue {
    pub fn new() -> Box<dyn Property<Position>> {
        Box::new(Self {})
    }
}

impl Property<Position> for MarketValue {
    fn header(&self) -> String {
        "Market value".into()
    }

    fn format_data(&self, t: &Position) -> String {
        format!(
            "{:.2}",
            t.amount() as f32 * t.instrument().price() * t.instrument().info().factor() as f32
        )
    }
}
