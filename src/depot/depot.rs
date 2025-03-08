use crate::instruments::{HasInstrument, InstrumentList, InstrumentSpec, MValue};

use super::{Event, Position, Trade, TransactionT};

#[derive(Debug, Default)]
pub struct Depot {
    positions: Vec<Position>,
    cash: MValue,
    instruments: InstrumentList,
    events: Vec<Event>,
    trades: Vec<Trade>,
}

impl Depot {
    pub fn instruments(&self) -> &InstrumentList {
        &self.instruments
    }

    pub fn positions(&self) -> &Vec<Position> {
        &self.positions
    }

    pub fn trades(&self) -> &Vec<Trade> {
        &self.trades
    }

    pub fn _deposit(&mut self, amount: MValue) {
        assert!(amount > 0.);
        self.cash += amount;
    }

    pub fn add_events(&mut self, mut events: Vec<Event>) {
        if events.is_empty() {
            return;
        }
        let first_new_ev = events.iter().map(|ev| ev.date()).min().unwrap();
        if self
            .events
            .last()
            .map_or(false, |last_ev| last_ev.date() > first_new_ev)
        {
            events.append(&mut self.events);
            self.trades.clear();
        }
        events.sort_by_key(|ev| ev.date());
        for ev in &events {
            self.apply_event(ev.clone());
        }
        self.events.append(&mut events);
    }

    fn apply_event(&mut self, ev: Event) {
        let instr = ev.instrument().clone();
        let old_instr = instr.clone();
        assert!(old_instr.eq(&self.instruments.add_or_get(instr)));

        let mut related = Vec::new();
        old_instr.info().get_related_instruments(&mut related);

        for i in related {
            let old_instr = i.clone();
            assert!(old_instr.eq(&self.instruments.add_or_get(i)));
        }

        match ev {
            Event::Transaction(tx) => {
                self.cash -= tx.total_cost();
                if let Some(i) = self
                    .positions
                    .iter()
                    .position(|pos| pos.instrument().eq(tx.instrument()))
                {
                    let pos = &mut self.positions[i];
                    if let Some(trade) = pos.apply_transaction(tx) {
                        self.trades.push(trade);
                    }
                    if pos.is_empty() {
                        self.positions.remove(i);
                    }
                } else {
                    self.positions.push(tx.into());
                }
            }
            Event::Split(spl) => {
                if let Some(pos) = self
                    .positions
                    .iter_mut()
                    .find(|pos| pos.instrument().eq(spl.instrument()))
                {
                    pos.apply_split(spl.factor);
                }
            }
        }
    }
}
