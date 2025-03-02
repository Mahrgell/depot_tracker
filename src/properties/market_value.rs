use eframe::egui;

use crate::instruments::MValue;

use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct MarketValue {
    acc: MValue,
}

impl MarketValue {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self::default())
    }
}

impl PropertyValue for MarketValue {
    type Value = MValue;
}

impl<T> FormattedProperty<T> for MarketValue
where
    T: Property<MarketValue>,
{
    fn header(&self) -> String {
        "Market Value".into()
    }

    fn format_data(&mut self, t: &T) -> String {
        let val = t.get(&self);
        self.acc += val;
        format!("{:.2}", val)
    }

    fn layout(&self) -> egui::Layout {
        egui::Layout::right_to_left(egui::Align::Center)
    }

    fn accumulated(&self) -> String {
        format!("{:.2}", self.acc)
    }
}
