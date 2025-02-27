use eframe::egui;

use crate::{
    depot::Depot,
    properties::{MarketValue, Name, PositionSize, Price},
};

use super::table_builder::build_table;

pub fn show(ui: &mut egui::Ui, depot: &Depot) {
    let props = vec![
        Name::new(),
        PositionSize::new(),
        Price::new(),
        MarketValue::new(),
    ];
    build_table(ui, depot.positions(), props);
}
