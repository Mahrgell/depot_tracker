use std::ops::RangeInclusive;

use chrono::{Datelike, NaiveDate};
use eframe::egui;
use egui_plot::{GridMark, Line, Plot, PlotPoints};

const LOCAL_STORAGE: &str = "stock_data_storage/";

use crate::{
    depot::Depot,
    instruments::InstrumentWrapped,
    stock_data::sources::{AlphaVantage, LocalFile},
};

#[derive(Default)]
pub struct Instruments {
    selected_index: usize,
    status_response: Option<String>,
}

impl Instruments {
    pub fn show(&mut self, ui: &mut egui::Ui, depot: &Depot) {
        let mut stocks: Vec<_> = depot
            .instruments()
            .get_list()
            .iter()
            .filter_map(|i| {
                if let InstrumentWrapped::Stock(stock) = i.info() {
                    Some((stock.symbol(), i))
                } else {
                    None
                }
            })
            .collect();
        if stocks.is_empty() {
            return;
        }

        stocks.sort_by_key(|(sym, _)| *sym);

        ui.horizontal(|ui| {
            egui::ComboBox::from_label("")
                .selected_text(stocks[self.selected_index].0)
                .show_ui(ui, |ui| {
                    for (index, &(sym, _)) in stocks.iter().enumerate() {
                        if ui
                            .selectable_label(self.selected_index == index, sym)
                            .clicked()
                        {
                            self.selected_index = index;
                            self.status_response = None;
                        }
                    }
                });
            if ui.button("Load all local").clicked() {
                let lf = LocalFile::new(LOCAL_STORAGE.into());
                let loaded_count = stocks
                    .iter()
                    .filter(|&(_, i)| i.update_data_with(&lf).is_ok())
                    .count();
                self.status_response =
                    Some(format!("Loaded data for {} instruments.", loaded_count));
            }
        });

        ui.separator();
        ui.label(format!(
            "Number of data points: {}",
            stocks[self.selected_index].1.data().nb_data_points()
        ));

        if ui.button("Load from AV").clicked() {
            let av = AlphaVantage::from_apikey_file("alphavantage.dtkey").unwrap();
            self.status_response =
                Some(match stocks[self.selected_index].1.update_data_with(&av) {
                    Ok((loaded_dp, new_dp)) => {
                        if new_dp > 0 {
                            let lf = LocalFile::new(LOCAL_STORAGE.into());
                            let _ = stocks[self.selected_index].1.save_data_local(&lf);
                        }
                        format!("Loaded {} data points. ({} new)", loaded_dp, new_dp)
                    }
                    Err(e) => format!("Failed to load due to {:?}", e),
                });
        }

        if let Some(sr) = self.status_response.as_ref() {
            ui.label(sr);
        }

        let data = stocks[self.selected_index].1.data();
        let raw = data.get_raw();
        if raw.is_empty() {
            return;
        }
        let min_date = raw.first().unwrap().date.num_days_from_ce() as f64;
        let max_date = raw.last().unwrap().date.num_days_from_ce() as f64;

        let min_price = raw.iter().map(|d| d.close).fold(f32::INFINITY, f32::min);
        let max_price = raw
            .iter()
            .map(|d| d.close)
            .fold(f32::NEG_INFINITY, f32::max);

        let points: PlotPoints = raw
            .iter()
            .map(|d| [d.date.num_days_from_ce() as f64, d.close as f64])
            .collect::<Vec<_>>()
            .into();

        let line = Line::new(points);

        let x_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
            let days = mark.value as i32;
            let date = NaiveDate::from_num_days_from_ce_opt(days).unwrap();
            date.format("%Y").to_string()
        };

        Plot::new("price_chart")
            .view_aspect(2.0)
            .include_x(min_date)
            .include_x(max_date)
            .include_y(min_price as f64)
            .include_y(max_price as f64)
            .x_axis_formatter(x_formatter)
            .x_grid_spacer(|gi| {
                let mut marks = Vec::new();
                let start = gi.bounds.0 as i32;
                let end = gi.bounds.1 as i32;
                let start_year = NaiveDate::from_num_days_from_ce_opt(start)
                    .unwrap()
                    .year_ce()
                    .1;
                let end_year = NaiveDate::from_num_days_from_ce_opt(end)
                    .unwrap()
                    .year_ce()
                    .1;
                for y in start_year..=end_year {
                    for m in [1] {
                        let date = NaiveDate::from_ymd_opt(y as i32, m, 1).unwrap();
                        let days = date.num_days_from_ce();
                        if days < start || days > end {
                            continue;
                        }
                        marks.push(GridMark {
                            value: days as f64,
                            step_size: if y % 5 == 0 { 5. * 365. } else { 1. * 365. },
                        });
                    }
                }

                marks
            })
            .label_formatter(|_name, value| {
                let days = value.x as i32;
                let date = NaiveDate::from_num_days_from_ce_opt(days).unwrap();
                format!("{}: {:.2}", date.format("%d.%m.%y").to_string(), value.y)
            })
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }
}
