use crate::tabs::Tab;
use crate::App;

pub struct TerminalTab;

impl Tab for TerminalTab {
    fn title(&self) -> &str {
        "Terminal"
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, _app: &mut App) {
        ui.horizontal(|ui| {
            ui.label("Aleksa");
        });
    }
}