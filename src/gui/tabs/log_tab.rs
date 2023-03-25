use super::Tab;

pub struct LogTab;

impl Tab for LogTab {
    fn show_ui(&self, app: &mut crate::gui::App, ui: &mut egui::Ui) {

    }

    fn title(&self) -> &str {
        "Log"
    }
}
