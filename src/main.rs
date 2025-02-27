mod depot;
mod instruments;
mod properties;
mod visu;

use visu::DepotTracker;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Depot Tracker",
        options,
        Box::new(|_cc| Ok(Box::new(DepotTracker::new()))),
    )
}
