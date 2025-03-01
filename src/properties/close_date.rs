use chrono::NaiveDate;

use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct CloseDate {}

impl CloseDate {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self {})
    }
}

impl PropertyValue for CloseDate {
    type Value = NaiveDate;
}

impl<T> FormattedProperty<T> for CloseDate
where
    T: Property<CloseDate>,
{
    fn header(&self) -> String {
        "Open Date".into()
    }

    fn format_data(&self, t: &T) -> String {
        t.get(&self).format("%d.%m.%y").to_string()
    }
}
