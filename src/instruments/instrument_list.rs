use std::rc::Rc;

use super::Instrument;

#[derive(Debug, Default)]
pub struct InstrumentList {
    instruments: Vec<Rc<Instrument>>,
}

impl InstrumentList {
    pub fn get_list(&self) -> &Vec<Rc<Instrument>> {
        &self.instruments
    }

    pub fn add_or_get(&mut self, instr: Rc<Instrument>) -> Rc<Instrument> {
        if let Some(existing) = self.instruments.iter().find(|&i| *i == instr) {
            existing.clone()
        } else {
            self.instruments.push(instr.clone());
            instr
        }
    }
}
