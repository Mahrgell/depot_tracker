use eframe::egui;

use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct PositionSize {}

impl PositionSize {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self {})
    }
}

impl PropertyValue for PositionSize {
    type Value = i32;
}

impl<T> FormattedProperty<T> for PositionSize
where
    T: Property<PositionSize>,
{
    fn header(&self) -> String {
        "Position".into()
    }

    fn format_data(&mut self, t: &T) -> String {
        t.get(&self).to_string()
    }

    fn layout(&self) -> egui::Layout {
        egui::Layout::right_to_left(egui::Align::Center)
    }
}
