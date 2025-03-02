use eframe::egui;

use crate::instruments::MValue;

use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct Profit {
    acc: MValue,
}

impl Profit {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self::default())
    }
}

impl PropertyValue for Profit {
    type Value = MValue;
}

impl<T> FormattedProperty<T> for Profit
where
    T: Property<Profit>,
{
    fn header(&self) -> String {
        "Price".into()
    }

    fn format_data(&mut self, t: &T) -> String {
        let val = t.get(&self);
        self.acc += val;
        format!("{:.2}", val)
    }

    fn long_data_example(&self) -> String {
        format!("{:.2}", -99999.99)
    }

    fn layout(&self) -> egui::Layout {
        egui::Layout::right_to_left(egui::Align::Center)
    }

    fn accumulated(&self) -> String {
        format!("{:.2}", self.acc)
    }
}
