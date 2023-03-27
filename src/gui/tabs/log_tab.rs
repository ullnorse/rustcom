use super::Tab;

pub struct LogTab;

impl Tab for LogTab {
    fn show_ui(&self, _app: &mut crate::gui::App, _ui: &mut egui::Ui) {

    }

    fn title(&self) -> &str {
        "Log"
    }
}
