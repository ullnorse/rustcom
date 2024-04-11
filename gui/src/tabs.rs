mod terminal_tab;

use egui_dock::DockState;

use terminal_tab::TerminalTab;

pub trait Tab {
    fn title(&self) -> &str;

    fn ui(&self, ui: &mut eframe::egui::Ui);
}



pub struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = Box<dyn Tab>;

    fn title(&mut self, tab: &mut Self::Tab) -> eframe::egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui);
    }
}

pub fn default_ui() -> egui_dock::DockState<Box<dyn Tab>> {
    DockState::new(vec![Box::new(TerminalTab)])
}