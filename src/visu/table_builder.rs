use eframe::egui;
use egui_extras::{Column, TableBuilder};

use crate::properties::Property;

pub fn build_table<T>(ui: &mut egui::Ui, data: &Vec<T>, props: Vec<Box<dyn Property<T>>>) {
    TableBuilder::new(ui)
        .striped(true)
        .columns(Column::auto(), props.len())
        .header(20.0, |mut header| {
            for p in &props {
                header.col(|ui| {
                    ui.label(p.header());
                });
            }
        })
        .body(|mut body| {
            for d in data {
                body.row(18.0, |mut row| {
                    for p in &props {
                        row.col(|ui| {
                            ui.label(p.format_data(d));
                        });
                    }
                });
            }
        });
}
