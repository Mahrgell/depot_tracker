use super::Instrument;

pub trait InstrumentSpec {
    fn as_instrument(self) -> Instrument;
    fn name(&self) -> &str;
}
