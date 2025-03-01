use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct Name {}

impl Name {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self {})
    }
}

impl PropertyValue for Name {
    type Value = String;
}

impl<T> FormattedProperty<T> for Name
where
    T: Property<Name>,
{
    fn header(&self) -> String {
        "Instrument".into()
    }

    fn format_data(&self, t: &T) -> String {
        t.get(&self)
    }
}
