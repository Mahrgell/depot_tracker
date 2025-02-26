use eframe::egui::{CentralPanel, Context, SidePanel};

use crate::{
    depot::{Depot, Transaction},
    instruments::Stock,
};

use super::Tab;

pub struct DepotTracker {
    depot: Depot,
    active_tab: Tab,
}

impl DepotTracker {
    pub fn new() -> Self {
        let mut depot = Depot::default();
        depot.deposit(5110.13);
        let tx = Transaction {
            amount: 10,
            instrument: depot.add_instrument(Stock::new("SPY".into()), 592.13),
            price: 590.05,
            fees: 13.6,
        };
        dbg!(&depot);
        depot.apply_transaction(&tx);
        dbg!(&depot);
        depot.apply_transaction(&tx);
        dbg!(&depot);

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
            self.active_tab.show(ui);
        });
    }
}
