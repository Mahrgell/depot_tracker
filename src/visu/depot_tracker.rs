use eframe::egui::{CentralPanel, Context, SidePanel};

use crate::depot::{Depot, IbkrParser};

use super::Tab;

pub struct DepotTracker {
    depot: Depot,
    active_tab: Tab,
}

impl DepotTracker {
    pub fn new() -> Self {
        let mut depot = Depot::default();

        let mut parser = IbkrParser::default();
        parser.parse("dummy.csv").unwrap();

        for tx in parser.transactions {
            depot.apply_transaction(&tx);
        }

        Self {
            depot,
            active_tab: Tab::Overview,
        }
    }
}

impl eframe::App for DepotTracker {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        SidePanel::left("tabs").show(ctx, |ui| {
            if ui.button("Overview").clicked() {
                self.active_tab = Tab::Overview;
            }
            if ui.button("Stocks").clicked() {
                self.active_tab = Tab::Stocks;
            }
            if ui.button("Chart").clicked() {
                self.active_tab = Tab::Chart;
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            self.active_tab.show(ui, &self.depot);
        });
    }
}
