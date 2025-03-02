use eframe::egui;

use crate::{
    depot::{Depot, Trade},
    instruments::{HasInstrument, InstrumentSpec},
    properties::{CloseDate, InstrumentName, OpenDate, Profit},
    visu::build_table,
};

#[derive(Debug, Default)]
pub struct Trades {
    filter_symbol: String,
    filter_include_underlying: bool,
}

impl Trades {
    pub fn new() -> Self {
        Self {
            filter_include_underlying: true,
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, depot: &Depot) {
        let props = vec![
            InstrumentName::fmt(),
            OpenDate::fmt(),
            CloseDate::fmt(),
            Profit::fmt(),
        ];
        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut self.filter_symbol);
        });

        ui.checkbox(&mut self.filter_include_underlying, "incl. underlying");

        ui.separator();
        let trades: Box<dyn Iterator<Item = &Trade>> = if self.filter_symbol.is_empty() {
            Box::new(depot.trades().iter())
        } else {
            Box::new(depot.trades().iter().filter(|t| {
                t.instrument()
                    .info()
                    .matches_symbol(&self.filter_symbol, self.filter_include_underlying)
            }))
        };
        build_table(ui, trades, props);
    }
}
