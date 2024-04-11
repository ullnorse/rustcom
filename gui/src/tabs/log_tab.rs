use crate::tabs::Tab;
use crate::App;
pub struct LogTab;

impl Tab for LogTab {
    fn title(&self) -> &str {
        "Log"
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, _app: &mut App) {
        ui.label("Log");
    }
}