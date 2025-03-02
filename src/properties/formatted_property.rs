use eframe::egui;

pub trait FormattedProperty<Target> {
    fn header(&self) -> String;
    fn format_data(&self, t: &Target) -> String;
    fn long_data_example(&self) -> String {
        self.header()
    }
    fn layout(&self) -> eframe::egui::Layout {
        egui::Layout::left_to_right(egui::Align::Center)
    }
}
