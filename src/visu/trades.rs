use eframe::egui;

use crate::{
    depot::Depot,
    properties::{CloseDate, InstrumentName, OpenDate, Profit},
};

use super::build_table;

pub fn show(ui: &mut egui::Ui, depot: &Depot) {
    let props = vec![
        InstrumentName::fmt(),
        OpenDate::fmt(),
        CloseDate::fmt(),
        Profit::fmt(),
    ];
    build_table(ui, depot.trades(), props);
}
