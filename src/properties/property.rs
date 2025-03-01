pub trait PropertyValue {
    type Value;
}

pub trait Property<Prop: PropertyValue> {
    fn get(&self, t: &Prop) -> Prop::Value;
}
