use super::InstrumentWrapped;

pub trait InstrumentSpec {
    fn as_wrapped(self) -> InstrumentWrapped;
    fn name(&self) -> &str;
    fn factor(&self) -> u32;
    fn matches_symbol(&self, symbol: &str, include_underlying: bool) -> bool;
}
