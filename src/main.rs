mod app;
mod graph;
mod node;
mod ui;

use app::AppState;
use eframe::Error;

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions::default();
    let app_state = AppState::new().map_err(|e| {
        eprintln!("Failed to initialize AppState: {:?}", e);
        Error::AppCreation(e.into())
    })?;

    eframe::run_native(
        "Brainbox - Second Brain",
        options,
        Box::new(|_cc| Ok(Box::new(app_state))),
    )
}
