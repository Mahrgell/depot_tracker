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
        "Close Date".into()
    }

    fn format_data(&self, t: &T) -> String {
        t.get(&self).format("%d.%m.%y").to_string()
    }

    // fn long_data_example(&self) -> String {
    //     static DATE: NaiveDate = NaiveDate::from_ymd_opt(2222, 12, 22).unwrap();
    //     format!("{}", DATE.format("%d.%m.%y"))
    // }
}
