use egui_dock::{TabViewer, Tree};
use egui::{ScrollArea, TextEdit, Layout, Align};
use super::gui::{App, Message};
use super::gui::port_settings::PortSettingsWindow;
use super::gui::line_end_picker::LineEndPicker;

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
                ui.horizontal(|ui| {
                    if self.device_connected {
                        if ui.button("Disconnect").clicked() {
                            self.do_update(Message::Disconnect);
                            self.device_connected = false;
                        }
                    } else if ui.button("Connect").clicked() {
                        self.do_update(Message::Connect);
                        self.device_connected = true;
                    }

                    if ui.button("Record").clicked() {

                    }

                    ui.checkbox(&mut self.timestamp, "Time").on_hover_text("Show time in receive box");
                    ui.checkbox(&mut self.lock_scrolling, "Lock scrolling");
                });

                ui.vertical(|ui| {
                    ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.horizontal(|ui| {
                            ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                                ui.add(LineEndPicker::new(70f32, &mut self.line_end));

                                if ui.button("Clear").clicked() {
                                    self.do_update(Message::ClearReceiveText);
                                }

                                if ui.button("Send").clicked() {
                                    let mut s = self.transmit_text.clone();
                                    s.push_str(self.line_end.into());

                                    self.do_update(Message::DataForTransmit(s));
                                }

                                ui.add_sized(ui.available_size(), TextEdit::singleline(&mut self.transmit_text));
                            });
                        });

                        ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .stick_to_bottom(self.lock_scrolling)
                            .show(ui, |ui| {
                                ui.with_layout(Layout::left_to_right(Align::Center).with_cross_justify(true), |ui| {
                                    ui.add_sized(ui.available_size(), TextEdit::multiline(&mut self.receive_text).interactive(false))
                                });
                            });
                    });
                });
            }

            Tabs::Settings => {
                let mut local_echo = false;

                ui.add(PortSettingsWindow::new(
                    &mut self.current_serial_device,
                    &self.serial_devices ,
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