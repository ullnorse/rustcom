use super::Tab;

pub struct LogTab;

impl Tab for LogTab {
    fn show_ui(&self, _app: &mut crate::gui::App, ui: &mut egui::Ui) {
        ui.label("Add log");
    }

    fn title(&self) -> &str {
        "Log"
    }
}
