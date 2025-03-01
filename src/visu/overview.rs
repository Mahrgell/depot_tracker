use eframe::egui;

use crate::{
    depot::Depot,
    properties::{InstrumentName, MarketValue, PositionSize, Price},
};

use super::build_table;

pub fn show(ui: &mut egui::Ui, depot: &Depot) {
    let props = vec![
        InstrumentName::fmt(),
        PositionSize::fmt(),
        Price::fmt(),
        MarketValue::fmt(),
    ];
    build_table(ui, depot.positions(), props);
}
