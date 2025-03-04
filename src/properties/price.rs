use eframe::egui;

use crate::instruments::MValue;

use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct Price {}

impl Price {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self {})
    }
}

impl PropertyValue for Price {
    type Value = Option<MValue>;
}

impl<T> FormattedProperty<T> for Price
where
    T: Property<Price>,
{
    fn header(&self) -> String {
        "Price".into()
    }

    fn format_data(&mut self, t: &T) -> String {
        let val = t.get(&self);
        match val {
            Some(val) => format!("{:.2}", val),
            None => String::new(),
        }
    }

    fn long_data_example(&self) -> String {
        format!("{:.2}", 99999.99)
    }

    fn layout(&self) -> egui::Layout {
        egui::Layout::right_to_left(egui::Align::Center)
    }
}
