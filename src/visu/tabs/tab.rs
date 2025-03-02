use eframe::egui::{self, Color32, Stroke, Vec2};

use crate::depot::Depot;

use super::Trades;

pub enum Tab {
    Overview,
    Trades(Trades),
    Chart,
}

impl Tab {
    pub fn show(&mut self, ui: &mut egui::Ui, depot: &Depot) {
        match self {
            Tab::Overview => super::overview::show(ui, depot),
            Tab::Trades(t) => t.show(ui, depot),
            Tab::Chart => show_chart(ui),
        }
    }
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
