use eframe::egui;

use crate::instruments::{HasInstrument, InstrumentSpec};

#[derive(Debug)]
pub struct SymbolFilter {
    symbol: String,
    include_underlying: bool,
}

impl SymbolFilter {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut self.symbol);
            ui.checkbox(&mut self.include_underlying, "incl. underlying");
        });

        ui.separator();
    }

    pub fn apply<'a, T: HasInstrument>(
        &'a self,
        iter: impl Iterator<Item = &'a T> + 'a,
    ) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        if self.symbol.is_empty() {
            Box::new(iter)
        } else {
            Box::new(iter.filter(|t| {
                t.instrument()
                    .info()
                    .matches_symbol(&self.symbol, self.include_underlying)
            }))
        }
    }
}

impl Default for SymbolFilter {
    fn default() -> Self {
        Self {
            symbol: Default::default(),
            include_underlying: true,
        }
    }
}
