use egui_dock::{TabViewer, Tree};
use egui::{ScrollArea, TextEdit, Layout};
use super::gui::App;
use super::gui::port_settings::PortSettingsWindow;
use super::gui::line_end_picker::{LineEnd, LineEndPicker};

#[derive(Debug)]
pub enum Tabs {
    Info,
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
                ui.horizontal(|ui| {
                    ui.button("Connect");
                    ui.button("Record");

                    let mut time = false;
                    ui.checkbox(&mut time, "Time");

                    let mut lock_scrolling = true;
                    ui.checkbox(&mut lock_scrolling, "Lock scrolling");
                });

                ui.vertical(|ui| {
                    ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.horizontal(|ui| {
                            ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                                let mut line_end = LineEnd::CR;

                                ui.add(LineEndPicker::new(70f32, &mut line_end));

                                ui.button("button1");
                                ui.button("button2");
                                let mut s = String::new();

                                ui.add_sized(ui.available_size(), TextEdit::singleline(&mut s));
                            });
                        });


                        ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .stick_to_bottom(true)
                            .show_viewport(ui, |ui, _viewport| {
                                ui.add_sized(ui.available_size(), TextEdit::multiline(&mut self.terminal_text).interactive(false))
                            });
                    });
                });
            }

            Tabs::Settings => {
                let devices = vec!["COM1".to_string(), "COM2".to_string()];
                let mut local_echo = false;

                ui.add(PortSettingsWindow::new(&mut self.device,
                    &devices ,
                    &mut self.serial_config.baudrate,
                    &mut self.serial_config.char_size,
                    &mut self.serial_config.stop_bits,
                    &mut self.serial_config.parity,
                    &mut self.serial_config.flow_control,
                    &mut local_echo));
            }

            _ => {}
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.to_string().into()
    }


}