mod settings_tab;
mod terminal_tab;

use egui_dock::{TabViewer, Tree};
use super::{App, Message};
use super::widgets::port_settings::PortSettingsWindow;

use settings_tab::render_settings_tab;
use terminal_tab::render_terminal_tab;

#[derive(Debug)]
pub enum Tabs {
    Terminal,
    Log,
    Settings,
}

impl std::fmt::Display for Tabs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn default_ui() -> Tree<Tabs> {
    let mut tree = Tree::new(vec![Tabs::Terminal]);
    let [main, side] = tree.split_right(0.into(), 0.8, vec![Tabs::Settings]);
    let [_side_top, _side_bottom] = tree.split_below(side, 0.6, vec![Tabs::Log]);
    tree.set_focused_node(main);
    tree
}

impl TabViewer for App {
    type Tab = Tabs;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.set_enabled(true);

        match tab {
            Tabs::Terminal => {
                render_terminal_tab(self, ui);
            }

            Tabs::Settings => {
                render_settings_tab(self, ui);
            }

            _ => {}
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.to_string().into()
    }
}
