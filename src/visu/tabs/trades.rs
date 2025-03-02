use eframe::egui;

use crate::{
    depot::Depot,
    properties::{CloseDate, InstrumentName, OpenDate, Profit},
    visu::{build_table, filters::SymbolFilter},
};

#[derive(Debug, Default)]
pub struct Trades {
    symbol_filter: SymbolFilter,
}

impl Trades {
    pub fn show(&mut self, ui: &mut egui::Ui, depot: &Depot) {
        let props = vec![
            InstrumentName::fmt(),
            OpenDate::fmt(),
            CloseDate::fmt(),
            Profit::fmt(),
        ];
        self.symbol_filter.show(ui);
        let trades = self.symbol_filter.apply(depot.trades().iter());
        build_table(ui, trades, props);
    }
}
