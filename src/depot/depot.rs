use crate::instruments::{DataProvider, InstrumentId, InstrumentList, InstrumentSpec, MValue};

use super::{Position, Transaction};

#[derive(Debug, Default)]
pub struct Depot {
    positions: Vec<Position>,
    cash: MValue,
    instruments: InstrumentList,
    data_provider: DataProvider,
}

impl Depot {
    pub fn instruments(&self) -> &InstrumentList {
        &self.instruments
    }

    pub fn data_provider(&self) -> &DataProvider {
        &self.data_provider
    }

    pub fn deposit(&mut self, amount: MValue) {
        assert!(amount > 0.);
        self.cash += amount;
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) {
        if let Some(i) = self
            .positions
            .iter()
            .position(|pos| pos.instrument == tx.instrument)
        {
            let pos = &mut self.positions[i];
            if pos.amount != -tx.amount {
                pos.amount += tx.amount;
            } else {
                self.positions.remove(i);
            }
        } else {
            self.positions.push(Position::from_transaction(&tx));
        }
        self.cash -= tx.amount as f32 * tx.price + tx.fees;
    }

    pub fn add_instrument<I: InstrumentSpec>(&mut self, instr: I, price: MValue) -> InstrumentId {
        let id = self.instruments.add(instr);
        self.data_provider.add_with_price(price);
        id
    }
}
