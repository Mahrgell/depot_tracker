use eframe::egui;

use crate::{
    depot::Depot,
    properties::{CloseDate, InstrumentName, OpenDate, PositionSize, Profit},
    visu::{
        build_table,
        filters::{InstrumentTypeFilter, SymbolFilter},
    },
};

#[derive(Debug, Default)]
pub struct Trades {
    instrument_type_filter: InstrumentTypeFilter,
    symbol_filter: SymbolFilter,
}

impl Trades {
    pub fn show(&mut self, ui: &mut egui::Ui, depot: &Depot) {
        let props = vec![
            InstrumentName::fmt(),
            PositionSize::fmt(),
            OpenDate::fmt(),
            CloseDate::fmt(),
            Profit::fmt(),
        ];
        self.instrument_type_filter.show(ui);
        self.symbol_filter.show(ui);
        let trades = self.instrument_type_filter.apply(depot.trades().iter());
        let trades = self.symbol_filter.apply(trades);
        build_table(ui, trades, props);
    }
}
