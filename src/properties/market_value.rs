use crate::instruments::MValue;

use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct MarketValue {}

impl MarketValue {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self {})
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

    fn format_data(&self, t: &T) -> String {
        format!("{:.2}", t.get(&self))
    }
}
