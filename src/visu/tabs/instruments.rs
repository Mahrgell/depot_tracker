use eframe::egui;

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
    }
}
