use eframe::egui;

use crate::instruments::{HasInstrument, InstrumentWrapped};

#[derive(Debug)]
pub struct InstrumentTypeFilter {
    stocks: bool,
    options: bool,
}

impl InstrumentTypeFilter {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.stocks, "Stocks");
            ui.checkbox(&mut self.options, "Options");
        });

        ui.separator();
    }

    pub fn apply<'a, T: HasInstrument>(
        &'a self,
        iter: impl Iterator<Item = &'a T> + 'a,
    ) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        Box::new(iter.filter(|t| match t.instrument().info() {
            InstrumentWrapped::Stock(_) => self.stocks,
            InstrumentWrapped::StockOption(_) => self.options,
        }))
    }
}

impl Default for InstrumentTypeFilter {
    fn default() -> Self {
        Self {
            stocks: true,
            options: true,
        }
    }
}
