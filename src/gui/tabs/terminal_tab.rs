use egui::Ui;
use egui::{ScrollArea, TextEdit, Layout, Align};
use crate::gui::Message;
use crate::gui::widgets::line_end_picker::LineEndPicker;
use super::App;
use super::Tab;

pub struct TerminalTab;

impl Tab for TerminalTab {
    fn show_ui(&self, app: &mut App, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if app.device_connected {
                if ui.button("Disconnect").clicked() {
                    app.do_update(Message::Disconnect);
                    app.device_connected = false;
                }
            } else if ui.button("Connect").clicked() {
                app.do_update(Message::Connect);
                app.device_connected = true;
            }

            if ui.button("Record").clicked() {

            }

            ui.checkbox(&mut app.timestamp, "Time").on_hover_text("Show time in receive box");
            ui.checkbox(&mut app.lock_scrolling, "Lock scrolling");
        });

        ui.vertical(|ui| {
            ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.add(LineEndPicker::new(70f32, &mut app.line_end));

                        if ui.button("Clear").clicked() {
                            app.do_update(Message::ClearReceiveText);
                        }

                        if ui.button("Send").clicked() {
                            let mut s = app.transmit_text.clone();
                            s.push_str(app.line_end.into());

                            app.do_update(Message::DataForTransmit(s));
                        }

                        ui.add_sized(ui.available_size(), TextEdit::singleline(&mut app.transmit_text));
                    });
                });

                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(app.lock_scrolling)
                    .show(ui, |ui| {
                        ui.with_layout(Layout::left_to_right(Align::Center).with_cross_justify(true), |ui| {
                            ui.add_sized(ui.available_size(), TextEdit::multiline(&mut app.receive_text).interactive(false))
                        });
                    });
            });
        });
    }

    fn title(&self) -> &str {
        "Terminal"
    }
}
