use eframe::egui;
use egui_extras::{Column, TableBuilder};

use crate::properties::FormattedProperty;

pub fn build_table<'a, T: 'a, I>(
    ui: &mut egui::Ui,
    data: I,
    mut props: Vec<Box<dyn FormattedProperty<T>>>,
) where
    I: IntoIterator<Item = &'a T>,
{
    let font_id = egui::TextStyle::Body.resolve(ui.style());

    let column_widths: Vec<f32> = props
        .iter()
        .map(|p| {
            ui.fonts(|f| {
                f.layout_no_wrap(p.long_data_example(), font_id.clone(), egui::Color32::WHITE)
            })
            .size()
            .x + 10.0
        })
        .collect();

    let mut table = TableBuilder::new(ui).striped(true);

    for &width in &column_widths {
        table = table.column(Column::initial(width).at_least(50.0));
    }

    table
        .header(20.0, |mut header| {
            for p in &props {
                header.col(|ui| {
                    ui.label(p.header());
                });
            }
        })
        .body(|mut body| {
            for d in data.into_iter() {
                body.row(18.0, |mut row| {
                    for p in &mut props {
                        row.col(|ui| {
                            ui.with_layout(p.layout(), |ui| {
                                ui.label(p.format_data(d));
                            });
                        });
                    }
                });
            }
            body.row(3.0, |mut row| {
                for _ in &props {
                    row.col(|ui| {
                        ui.separator();
                    });
                }
            });
            body.row(18.0, |mut row| {
                for p in &mut props {
                    row.col(|ui| {
                        ui.with_layout(p.layout(), |ui| {
                            ui.label(p.accumulated());
                        });
                    });
                }
            });
        });
}
