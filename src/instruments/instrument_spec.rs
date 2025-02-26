use super::InstrumentWrapped;

pub trait InstrumentSpec {
    fn as_wrapped(self) -> InstrumentWrapped;
    fn name(&self) -> &str;
}
