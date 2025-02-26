use chrono::NaiveDate;
use eframe::egui::{CentralPanel, Context, SidePanel};

use crate::{
    depot::{Depot, Transaction},
    instruments::{Instrument, OptionType, Stock, StockOption},
};

use super::Tab;

pub struct DepotTracker {
    depot: Depot,
    active_tab: Tab,
}

impl DepotTracker {
    pub fn new() -> Self {
        let mut depot = Depot::default();
        depot.deposit(10000.);
        let spy = Instrument::new(Stock::new("SPY".into()), 592.13);
        let tx1 = Transaction {
            amount: 10,
            instrument: spy.clone(),
            price: 590.00,
            fees: 13.6,
        };
        dbg!(&depot);
        depot.apply_transaction(&tx1);
        dbg!(&depot);
        let tx2 = Transaction {
            amount: -2,
            instrument: Instrument::new(StockOption::new(OptionType::Put, spy.clone(), 585., 100, NaiveDate::from_ymd_opt(2025, 03, 25).unwrap()),1.12),
            price: 1.0,
            fees: 4.,
        };
        
        depot.apply_transaction(&tx2);
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
