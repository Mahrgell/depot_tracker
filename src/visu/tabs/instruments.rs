use eframe::egui;

use crate::{depot::Depot, instruments::InstrumentWrapped, stock_data::sources::LocalFile};

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

        ui.separator();
        ui.label(format!(
            "Number of data points: {}",
            stocks[self.selected_index].1.data().nb_data_points()
        ));
        const LOCAL_STORAGE: &str = "stock_data_storage/";
        if ui.button("Load local").clicked() {
            let lf = LocalFile::new(LOCAL_STORAGE.into());
            self.status_response =
                Some(match stocks[self.selected_index].1.update_data_with(&lf) {
                    Ok((loaded_dp, new_dp)) => {
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
