use eframe::egui::{self, Color32, Stroke, Vec2};
use egui_extras::{Column, TableBuilder};

use crate::depot::Depot;

pub enum Tab {
    Overview,
    Stocks,
    Chart,
}

impl Tab {
    pub fn show(&self, ui: &mut egui::Ui, depot: &Depot) {
        match self {
            Tab::Overview => super::overview::show(ui, depot),
            Tab::Stocks => show_stock_table(ui),
            Tab::Chart => show_chart(ui),
        }
    }
}

fn show_stock_table(ui: &mut egui::Ui) {
    let stocks = vec![
        ("AAPL", 150.0, 10),
        ("GOOGL", 2800.0, 5),
        ("AMZN", 3400.0, 2),
    ];

    TableBuilder::new(ui)
        .striped(true)
        .columns(Column::auto(), 3)
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.label("Stock");
            });
            header.col(|ui| {
                ui.label("Price ($)");
            });
            header.col(|ui| {
                ui.label("Quantity");
            });
        })
        .body(|mut body| {
            for (name, price, quantity) in stocks {
                body.row(18.0, |mut row| {
                    row.col(|ui| {
                        ui.label(name);
                    });
                    row.col(|ui| {
                        ui.label(format!("{:.2}", price));
                    });
                    row.col(|ui| {
                        ui.label(quantity.to_string());
                    });
                });
            }
        });
}

fn show_chart(ui: &mut egui::Ui) {
    let (response, painter) = ui.allocate_painter(Vec2::new(300.0, 150.0), egui::Sense::hover());
    let rect = response.rect;
    painter.line_segment(
        [rect.left_top(), rect.right_bottom()],
        Stroke::new(1., Color32::RED),
    );
    painter.line_segment(
        [rect.right_top(), rect.left_bottom()],
        Stroke::new(1., Color32::BLUE),
    );
    ui.label("Custom chart (placeholder)");
}
