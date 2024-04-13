use crate::tabs::Tab;
use crate::App;

use base::messages::{self, Message};
use eframe::egui::{Align, Layout, ScrollArea, TextEdit};
use crate::widgets::line_end_picker::LineEndPicker;

pub struct TerminalTab;

impl Tab for TerminalTab {
    fn title(&self) -> &str {
        "Terminal"
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, app: &mut App) {
        ui.horizontal(|ui| {
            if app.device_connected {
                if ui.button("Disconnect").clicked() {
                    messages::send(Message::Disconnect);
                }
            } else if ui.button("Connect").clicked() {
                messages::send(Message::Connect);
            }

            if !app.recording_started {
                if ui.button("Record").clicked() {
                    messages::send(Message::StartRecording);
                }
            } else if ui.button("Stop recording").clicked() {
                messages::send(Message::StopRecording);
            }

            if ui.button("Clear").clicked() {
                messages::send(Message::ClearTerminalText);
            }

            ui.checkbox(&mut app.timestamp, "Time").on_hover_text("Show time in receive box");
            ui.checkbox(&mut app.lock_scrolling, "Lock scrolling");
        });

        ui.vertical(|ui| {
            ui.with_layout(Layout::bottom_up(egui_dock::egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(egui_dock::egui::Align::Max), |ui| {
                        // ui.add(FileProtocolPicker::new(80f32, &mut app.file_protocol));

                        // if ui.button("Send file...").clicked() {

                        // }

                        ui.add(LineEndPicker::new(70f32, &mut app.line_end));

                        ui.add_sized(ui.available_size(), TextEdit::singleline(&mut app.transmit_text));
                    });
                });

                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(app.lock_scrolling)
                    .show(ui, |ui| {
                        ui.with_layout(Layout::left_to_right(Align::Center).with_cross_justify(true), |ui| {
                            ui.add_sized(ui.available_size(), TextEdit::multiline(&mut app.terminal_text).interactive(false))
                        });
                    });
            });
        });
    }
}