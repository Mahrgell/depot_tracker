use crate::depot::Position;

use super::Property;

pub struct PositionSize {}

impl PositionSize {
    pub fn new() -> Box<dyn Property<Position>> {
        Box::new(Self {})
    }
}

impl Property<Position> for PositionSize {
    fn header(&self) -> String {
        "Position".into()
    }

    fn format_data(&self, t: &Position) -> String {
        t.amount().to_string()
    }
}
