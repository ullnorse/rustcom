use crate::tabs::Tab;

pub struct TerminalTab;

impl Tab for TerminalTab {
    fn title(&self) -> &str {
        "Terminal"
    }

    fn ui(&self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Aleksa");
        });
    }
}