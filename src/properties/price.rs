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
    type Value = MValue;
}

impl<T> FormattedProperty<T> for Price
where
    T: Property<Price>,
{
    fn header(&self) -> String {
        "Price".into()
    }

    fn format_data(&self, t: &T) -> String {
        format!("{:.2}", t.get(&self))
    }
}
