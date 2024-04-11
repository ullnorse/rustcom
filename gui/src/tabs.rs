mod log_tab;
mod settings_tab;
mod terminal_tab;

use egui_dock::DockState;

use log_tab::LogTab;
use settings_tab::SettingsTab;
use terminal_tab::TerminalTab;

use crate::App;

pub trait Tab {
    fn title(&self) -> &str;

    fn ui(&mut self, ui: &mut eframe::egui::Ui, app: &mut App);
}

pub fn default_ui() -> egui_dock::DockState<Box<dyn Tab>> {
    let mut tree: DockState<Box<dyn Tab>> = DockState::new(vec![Box::new(TerminalTab)]);

    let [_, right] = tree.main_surface_mut().split_right(egui_dock::NodeIndex::root(), 0.75, vec![Box::new(SettingsTab)]);
    let [_, _] = tree.main_surface_mut().split_below(right, 0.5, vec![Box::new(LogTab)]);

    tree
}

impl egui_dock::TabViewer for App {
    type Tab = Box<dyn Tab>;

    fn title(&mut self, tab: &mut Self::Tab) -> eframe::egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui, self);
    }
}
