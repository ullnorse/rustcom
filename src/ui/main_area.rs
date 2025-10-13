use eframe::egui::{Button, CentralPanel, ComboBox, Context, Key, Modifiers, ScrollArea, TextEdit, Ui, Align, Layout};
use log::{error, info};

use crate::app::App;
use crate::serial::{DataBits, FlowControl, Parity, SerialMainState, StopBits};
use crate::ui;

impl App {
    pub fn render_main_area(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.render_settings_ui(ui);

            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                self.render_input_ui(ui, ctx);
                self.render_output_ui(ui);
            });
        });
    }

    fn render_output_ui(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear").clicked() {
                    self.output_text.clear();
                }

                ui.checkbox(&mut self.hex_output, "Hex output");
            });

            let selectable_text = |ui: &mut Ui, mut text: &str| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add_sized(ui.available_size(), TextEdit::multiline(&mut text));
                });
            };

            ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(self.auto_scroll)
                .show(ui, |ui| {
                    selectable_text(ui, &mut self.output_text);
                });
        });
    }

    pub fn render_input_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.add_space(10f32);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label("Input: ");
                ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    if ui.button("Send").clicked() {
                        self.send();
                    }

                    let line_ends = [
                        ("", "None"),
                        ("\n", "+LF"),
                        ("\r", "+CR"),
                        ("\r\n", "+CRLF"),
                    ];

                    ComboBox::from_id_salt("ComboBox line end")
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
                        TextEdit::singleline(&mut self.input_text),
                    );

                    if response.lost_focus()
                        && ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Enter))
                    {
                        self.send();
                        response.request_focus();
                    }
                });
            });
        });
    }

    pub fn render_settings_ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if self.serial.is_some() {
                            if ui
                                .add_sized((80f32, 10f32), Button::new("Disconnect"))
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
                            .add_sized((80f32, 10f32), Button::new("Connect"))
                            .clicked()
                        {
                            match self.connect() {
                                Ok(_) => info!("Opened serial port"),
                                Err(e) => error!("Error connecting to serial port: {e:?}"),
                            }
                        }
                    });

                    ui.vertical(|ui| {
                        ui.checkbox(&mut self.auto_scroll, "Auto scroll")
                            .on_hover_text_at_pointer("Auto scroll receive box to the end");
                    });

                    ui.add_space(ui.available_width() - 180f32);

                    ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ComboBox::from_id_salt("COM Port")
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
}
