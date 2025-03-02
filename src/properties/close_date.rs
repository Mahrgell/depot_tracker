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
    type Value = (NaiveDate, bool);
}

impl<T> FormattedProperty<T> for CloseDate
where
    T: Property<CloseDate>,
{
    fn header(&self) -> String {
        "Close Date".into()
    }

    fn format_data(&mut self, t: &T) -> String {
        let (date, exp) = t.get(&self);
        format!("{}{}", date.format("%d.%m.%y"), if exp { "*" } else { "" })
    }

    // fn long_data_example(&self) -> String {
    //     static DATE: NaiveDate = NaiveDate::from_ymd_opt(2222, 12, 22).unwrap();
    //     format!("{}*", DATE.format("%d.%m.%y"))
    // }
}
