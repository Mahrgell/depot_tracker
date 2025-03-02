use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct InstrumentName {}

impl InstrumentName {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self {})
    }
}

impl PropertyValue for InstrumentName {
    type Value = String;
}

impl<T> FormattedProperty<T> for InstrumentName
where
    T: Property<InstrumentName>,
{
    fn header(&self) -> String {
        "Instrument".into()
    }

    fn format_data(&self, t: &T) -> String {
        t.get(&self)
    }

    fn long_data_example(&self) -> String {
        static EXAMPLE: &str = "Call XXXXX - 22.12.2222";
        EXAMPLE.into()
    }
}
