use eframe::egui;

use crate::{
    depot::Depot,
    properties::{MarketValue, Name, PositionSize, Price},
};

use super::build_table;

pub fn show(ui: &mut egui::Ui, depot: &Depot) {
    let props = vec![
        Name::fmt(),
        PositionSize::fmt(),
        Price::fmt(),
        MarketValue::fmt(),
    ];
    build_table(ui, depot.positions(), props);
}
