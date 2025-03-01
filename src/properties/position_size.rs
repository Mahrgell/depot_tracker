use super::{FormattedProperty, Property, PropertyValue};

#[derive(Default)]
pub struct PositionSize {}

impl PositionSize {
    pub fn fmt<T>() -> Box<dyn FormattedProperty<T>>
    where
        Self: FormattedProperty<T>,
    {
        Box::new(Self {})
    }
}

impl PropertyValue for PositionSize {
    type Value = i32;
}

impl<T> FormattedProperty<T> for PositionSize
where
    T: Property<PositionSize>,
{
    fn header(&self) -> String {
        "Position".into()
    }

    fn format_data(&self, t: &T) -> String {
        t.get(&self).to_string()
    }
}
