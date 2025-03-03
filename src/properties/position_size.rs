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
    type Value = f32;
}

impl<T> FormattedProperty<T> for PositionSize
where
    T: Property<PositionSize>,
{
    fn header(&self) -> String {
        "Position".into()
    }

    fn format_data(&mut self, t: &T) -> String {
        let val = t.get(&self);
        let val_i32 = val as i32;
        if val_i32 as f32 == val {
            val_i32.to_string()
        } else {
            format!("{:.2}", val)
        }
    }

    fn layout(&self) -> egui::Layout {
        egui::Layout::right_to_left(egui::Align::Center)
    }
}
