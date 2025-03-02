use eframe::egui;

use crate::{
    depot::Depot,
    instruments::{HasInstrument, InstrumentSpec},
    properties::{CloseDate, InstrumentName, OpenDate, Profit},
    visu::build_table,
};

pub fn show(ui: &mut egui::Ui, depot: &Depot) {
    let props = vec![
        InstrumentName::fmt(),
        OpenDate::fmt(),
        CloseDate::fmt(),
        Profit::fmt(),
    ];
    build_table(
        ui,
        depot
            .trades()
            .iter()
            .filter(|t| t.instrument().info().matches_symbol("TSLA", true)),
        props,
    );
}
