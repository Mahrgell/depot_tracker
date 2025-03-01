use chrono::NaiveDate;

use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct OpenDate {}

impl OpenDate {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self {})
    }
}

impl PropertyValue for OpenDate {
    type Value = (NaiveDate, Option<NaiveDate>);
}

impl<T> FormattedProperty<T> for OpenDate
where
    T: Property<OpenDate>,
{
    fn header(&self) -> String {
        "Open Date".into()
    }

    fn format_data(&self, t: &T) -> String {
        let (t1, t2) = t.get(&self);
        match t2 {
            Some(t2) => format!("{} - {}", t1.format("%d.%m.%y"), t2.format("%d.%m.%y")),
            None => t1.format("%d.%m.%y").to_string(),
        }
    }
}
