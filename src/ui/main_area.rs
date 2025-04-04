use directories::BaseDirs;
use egui::{Align, Layout};
use log::{error, info};
use rfd::FileDialog;

use crate::app::App;
use crate::macros::Macros;
use crate::serial::{DataBits, FlowControl, Parity, SerialMainState, SerialMsg, StopBits};
use crate::ui;

impl App {
    pub fn render_main_area(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_settings_ui(ui);

            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                self.render_input_ui(ui, ctx);
                self.render_macros_ui(ui);
                self.render_output_ui(ui);
            });
        });
    }

    fn render_output_ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear").clicked() {
                    self.output_text.clear();
                }

                ui.checkbox(&mut self.hex_output, "Hex output");

                if ui
                    .checkbox(&mut self.logging_to_file_started, "Logging to:")
                    .clicked()
                {
                    if self.logging_to_file_started {
                        self.stop_recording_thread();
                    } else {
                        self.start_recording_thread();
                    }
                }

                let selectable_text = |ui: &mut egui::Ui, mut text: &str| {
                    ui.add_sized(
                        [200.0, ui.available_height()],
                        egui::TextEdit::singleline(&mut text),
                    );
                };

                selectable_text(ui, &self.log_file_name);

                if ui
                    .button("...")
                    .on_hover_text_at_pointer("Choose log file via file chooser")
                    .clicked()
                {
                    if let Some(path) = BaseDirs::new()
                        .and_then(|dirs| {
                            FileDialog::new()
                                .set_title("Open")
                                .set_directory(dirs.home_dir())
                                .pick_file()
                        })
                        .and_then(|path| path.into_os_string().into_string().ok())
                    {
                        self.log_file_name = path;
                    }
                }

                ui.checkbox(&mut self.log_file_append, "Append")
                    .on_hover_text_at_pointer(
                        "Appends to an existing log file instead of truncating it",
                    );
            });

            let selectable_text = |ui: &mut egui::Ui, mut text: &str| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut text));
                });
            };

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(self.auto_scroll)
                .show(ui, |ui| {
                    selectable_text(ui, &mut self.output_text);
                });
        });
    }

    pub fn render_input_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.add_space(10f32);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label("Input: ");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                    if ui.button("Send").clicked() {
                        self.send();
                    }

                    let line_ends = [
                        ("", "None"),
                        ("\n", "+LF"),
                        ("\r", "+CR"),
                        ("\r\n", "+CRLF"),
                    ];

                    egui::ComboBox::from_id_salt("ComboBox line end")
                        .width(50f32)
                        .selected_text(
                            line_ends
                                .iter()
                                .find(|&&(value, _)| value == self.input_line_end)
                                .map_or("".to_string(), |&(_, label)| label.to_string()),
                        )
                        .show_ui(ui, |ui| {
                            for &(value, label) in &line_ends {
                                ui.selectable_value(
                                    &mut self.input_line_end,
                                    value.to_string(),
                                    label.to_string(),
                                );
                            }
                        });

                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.input_text),
                    );

                    if response.lost_focus()
                        && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
                    {
                        self.send();
                        response.request_focus();
                    }
                });
            });
        });
    }

    pub fn render_settings_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if self.serial.is_some() {
                            if ui
                                .add_sized((70f32, 10f32), egui::Button::new("Disconnect"))
                                .clicked()
                            {
                                match self.disconnect() {
                                    Ok(_) => {
                                        info!("Closed serial port: {}", self.serial_settings.port)
                                    }
                                    Err(e) => {
                                        error!("Error disconnecting from serial port: {e:?}")
                                    }
                                }
                            }
                        } else if ui
                            .add_sized((70f32, 10f32), egui::Button::new("Connect"))
                            .clicked()
                        {
                            match self.connect() {
                                Ok(_) => info!("Opened serial port"),
                                Err(e) => error!("Error connecting to serial port: {e:?}"),
                            }
                        }
                    });

                    ui.vertical(|ui| {
                        ui.checkbox(&mut false, "Timestamp")
                            .on_hover_text_at_pointer("Add timestamp to new lines in receive box");
                        ui.checkbox(&mut self.auto_scroll, "Auto scroll")
                            .on_hover_text_at_pointer("Auto scroll receive box to the end");
                    });

                    ui.add_space(ui.available_width() - 180f32);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                egui::ComboBox::from_id_salt("COM Port")
                                    .selected_text(&self.serial_settings.port)
                                    .show_ui(ui, |ui| {
                                        for device in &self.available_ports {
                                            ui.selectable_value(
                                                &mut self.serial_settings.port,
                                                device.clone(),
                                                device,
                                            );
                                        }
                                    });

                                if ui.button("Refresh").clicked() {
                                    self.available_ports = SerialMainState::available_ports();

                                    if !self.available_ports.is_empty() {
                                        self.serial_settings.port = self.available_ports[0].clone();
                                    }
                                }
                            });

                            let baud_rates = [
                                0, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 1000000,
                            ];

                            ui::render_combo_box(
                                ui,
                                "Baud rate",
                                &mut self.serial_settings.baud_rate,
                                &baud_rates,
                            );

                            let data_bits = [
                                DataBits::Five,
                                DataBits::Six,
                                DataBits::Seven,
                                DataBits::Eight,
                            ];

                            ui::render_combo_box(
                                ui,
                                "Data bits",
                                &mut self.serial_settings.data_bits,
                                &data_bits,
                            );

                            let parity_options = [Parity::None, Parity::Odd, Parity::Even];

                            ui::render_combo_box(
                                ui,
                                "Parity",
                                &mut self.serial_settings.parity,
                                &parity_options,
                            );

                            let stop_bits = [StopBits::One, StopBits::Two];

                            ui::render_combo_box(
                                ui,
                                "Stop bits",
                                &mut self.serial_settings.stop_bits,
                                &stop_bits,
                            );

                            let flow_control_options = [
                                FlowControl::None,
                                FlowControl::Hardware,
                                FlowControl::Software,
                            ];

                            ui::render_combo_box(
                                ui,
                                "Flow control",
                                &mut self.serial_settings.flow_control,
                                &flow_control_options,
                            );
                        });

                        ui.add_space(ui.available_width());
                    });
                });
            });
        });
    }

    pub fn render_macros_ui(&mut self, ui: &mut egui::Ui) {
        if self.macros_ui_open {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Macros");
                    if ui.button("Set Macros").clicked() {
                        self.macros_window_open = true;
                    }

                    for i in 0..Macros::NUM_OF_MACROS {
                        if ui
                            .button(format!("M{}{}", i + 1, if i < 10 { " " } else { "" }))
                            .clicked()
                        {
                            if let Some(m) = self.macros.get_macro(i) {
                                let text = m.text.clone();
                                if let Err(e) = self.serial_send(SerialMsg::Str(text + "\n")) {
                                    error!("Couldn't send macro: {e:?}");
                                }
                            }
                        }
                    }

                    ui.add_space(ui.available_width());
                });
            });
        }
    }
}
