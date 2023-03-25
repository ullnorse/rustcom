pub mod settings_tab;
mod terminal_tab;
pub mod log_tab;

use egui_dock::{TabViewer, Tree};
use super::App;
use egui::Ui;

use settings_tab::SettingsTab;
use terminal_tab::TerminalTab;
use log_tab::LogTab;

pub trait Tab {
    fn show_ui(&self, app: &mut App, ui: &mut Ui);
    fn title(&self) -> &str;
}

pub fn default_ui() -> Tree<Box<dyn Tab>> {
    let mut tree: Tree<Box<dyn Tab>> = Tree::new(vec![Box::new(TerminalTab)]);
    let [main, side] = tree.split_right(0.into(), 0.8, vec![Box::new(SettingsTab)]);
    let [_side_top, _side_bottom] = tree.split_below(side, 0.6, vec![Box::new(LogTab)]);
    tree.set_focused_node(main);
    tree
}

impl TabViewer for App {
    type Tab = Box<dyn Tab>;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.show_ui(self, ui);
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }
}
