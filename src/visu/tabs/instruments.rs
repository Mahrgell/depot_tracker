use eframe::egui;

use crate::{depot::Depot, instruments::InstrumentWrapped};

#[derive(Default)]
pub struct Instruments {
    selected_index: usize,
}

impl Instruments {
    pub fn show(&mut self, ui: &mut egui::Ui, depot: &Depot) {
        let stocks: Vec<_> = depot
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

        egui::ComboBox::from_label("")
            .selected_text(stocks[self.selected_index].0)
            .show_ui(ui, |ui| {
                for (index, &(sym, _)) in stocks.iter().enumerate() {
                    if ui
                        .selectable_label(self.selected_index == index, sym)
                        .clicked()
                    {
                        self.selected_index = index;
                    }
                }
            });

        ui.separator();
        ui.label(format!(
            "Number of data points: {}",
            stocks[self.selected_index]
                .1
                .data()
                .read()
                .unwrap()
                .nb_data_points()
        ));
    }
}
