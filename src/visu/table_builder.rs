use eframe::egui;
use egui_extras::{Column, TableBuilder};

use crate::properties::FormattedProperty;

pub fn build_table<'a, T: 'a, I>(
    ui: &mut egui::Ui,
    data: I,
    props: Vec<Box<dyn FormattedProperty<T>>>,
) where
    I: IntoIterator<Item = &'a T>,
{
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
            let mut itr = data.into_iter();
            while let Some(d) = itr.next() {
                body.row(18.0, |mut row| {
                    for p in &props {
                        row.col(|ui| {
                            ui.label(p.format_data(&d));
                        });
                    }
                });
            }
        });
}
