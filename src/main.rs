mod app;
mod node;
mod ui;

use app::AppState;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Brainbox - Second Brain",
        options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )
}
