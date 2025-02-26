use super::{InstrumentId, MValue};

#[derive(Debug, Default)]
pub struct DataProvider {
    data: Vec<MValue>,
}

impl DataProvider {
    pub fn add_with_price(&mut self, price: MValue) {
        self.data.push(price);
    }

    pub fn price(&self, id: InstrumentId) -> MValue {
        self.data[id]
    }
}
