#![allow(unused)]

mod status_bar;
mod menu_bar;
mod messages;
mod serial;
mod macros;


use clipboard::ClipboardProvider;
use crossbeam::channel::{Sender, Receiver, unbounded};
use egui::{Vec2, ViewportBuilder};
use messages::Message;
use serial::{Serial, SerialSettings};
use macros::Macros;

use serialport5::{DataBits, Parity, StopBits, FlowControl};

#[derive(PartialEq)]
enum TextMode {
    Hex,
    Ascii,
}

struct App {
    message_channel: (Sender<Message>, Receiver<Message>),

    serial_settings: SerialSettings,
    port: String,
    serial: Serial,

    input_text: String,
    input_line_end: String,
    output_text: String,

    auto_scroll: bool,
    hex_output: bool,
    text_mode: TextMode,

    tx_cnt: usize,
    rx_cnt: usize,

    macros: Macros,
}

impl App {
    fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Light);

        let mut app = Self {
            message_channel: unbounded(),
            serial_settings: SerialSettings::default(),
            port: String::new(),
            serial: Serial::new(),
            input_text: String::new(),
            input_line_end: String::new(),
            output_text: String::new(),
            auto_scroll: true,
            hex_output: false,
            text_mode: TextMode::Ascii,
            tx_cnt: 0,
            rx_cnt: 0,
            macros: Macros::new(),
        };

        let available_ports = Serial::available_ports();

        if !available_ports.is_empty() {
            app.port = available_ports[0].clone();
        }

        app
    }

    fn render_main_area(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if self.serial.is_connected() {
                                if ui.add_sized((70f32, 10f32), egui::Button::new("Disconnect")).clicked() {
                                    self.send_message(Message::TryDisconnect);
                                }
                            } else if ui.add_sized((70f32, 10f32), egui::Button::new("Connect")).clicked() {
                                self.send_message(Message::TryConnect);
                            }
                        });

                        ui.vertical(|ui| {
                            ui.checkbox(&mut false, "Timestamp").on_hover_text_at_pointer("Add timestamp to new lines in receive box");
                            ui.checkbox(&mut self.auto_scroll, "Auto scroll").on_hover_text_at_pointer("Auto scroll receive box to the end");
                        });

                        // ui.add_space(ui.available_width() - 180f32);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    let available_ports = Serial::available_ports();

                                    egui::ComboBox::from_id_salt("COM Port")
                                        .selected_text(&self.port)
                                        .show_ui(ui, |ui| {
                                            for device in &available_ports {
                                                ui.selectable_value(&mut self.port, device.clone(), device);
                                            }
                                        });

                                    if ui.button("Refresh").clicked() {
                                        let available_ports = Serial::available_ports();

                                        if !available_ports.is_empty() {
                                            self.port = available_ports[0].clone();
                                        }
                                    }

                                });

                                let baud_rates = [1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 1000000];
                                egui::ComboBox::from_label("Baud rate")
                                    .selected_text(self.serial_settings.baud_rate.to_string())
                                    .show_ui(ui, |ui| {
                                        for baud_rate in baud_rates {
                                            ui.selectable_value(&mut self.serial_settings.baud_rate, baud_rate, baud_rate.to_string());
                                        }
                                    });

                                let data_bits = [DataBits::Five, DataBits::Six, DataBits::Seven, DataBits::Eight];
                                egui::ComboBox::from_label("Data bits")
                                    .selected_text(match self.serial_settings.data_bits {
                                        DataBits::Five => "5",
                                        DataBits::Six => "6",
                                        DataBits::Seven => "7",
                                        DataBits::Eight => "8",
                                    })
                                    .show_ui(ui, |ui| {
                                        for bits in data_bits {
                                            ui.selectable_value(&mut self.serial_settings.data_bits, bits, match bits {
                                                DataBits::Five => "5",
                                                DataBits::Six => "6",
                                                DataBits::Seven => "7",
                                                DataBits::Eight => "8",
                                            });
                                        }
                                    });

                                let parity_options = [Parity::None, Parity::Odd, Parity::Even];
                                egui::ComboBox::from_label("Parity")
                                    .selected_text(format!("{:?}", self.serial_settings.parity))
                                    .show_ui(ui, |ui| {
                                        for parity in parity_options {
                                            ui.selectable_value(&mut self.serial_settings.parity, parity, format!("{:?}", parity));
                                        }
                                    });

                                let stop_bits_values = [StopBits::One, StopBits::Two];
                                egui::ComboBox::from_label("Stop bits")
                                    .selected_text(match self.serial_settings.stop_bits {
                                        StopBits::One => "1",
                                        StopBits::Two => "2",
                                    })
                                    .show_ui(ui, |ui| {
                                        for stop_bits in stop_bits_values {
                                            ui.selectable_value(&mut self.serial_settings.stop_bits, stop_bits, match stop_bits {
                                                StopBits::One => "1",
                                                StopBits::Two => "2",
                                            });
                                        }
                                    });

                                let flow_control_options = [FlowControl::None, FlowControl::Hardware, FlowControl::Software];
                                egui::ComboBox::from_label("Flowcontrol")
                                    .selected_text(format!("{:?}", self.serial_settings.flow_control))
                                    .show_ui(ui, |ui| {
                                        for flow_control in flow_control_options {
                                            ui.selectable_value(&mut self.serial_settings.flow_control, flow_control, format!("{:?}", flow_control));
                                        }
                                    });

                                });

                                ui.add_space(ui.available_width());
                        });
                    });
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.add_space(10f32);

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.label("Input: ");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            // egui::ComboBox::from_id_salt("ComboBox file type").show_ui(ui, |ui| {});
                            // ui.button("Send File...");
                            if ui.button("Send").clicked() {
                                self.send_message(Message::DataForTransmit);
                            }

                            let line_ends = ["".to_string(), "\n".to_string(), "\r".to_string(), "\r\n".to_string()];

                            egui::ComboBox::from_id_salt("ComboBox line end")
                                .width(50f32)
                                .selected_text(match self.input_line_end.as_str() {
                                    "" => "None".to_string(),
                                    "\n" => "+LF".to_string(),
                                    "\r" => "+CR".to_string(),
                                    "\r\n" => "+CRLF".to_string(),
                                    _ => "".to_string(),
                                })
                                .show_ui(ui, |ui| {
                                    for line_end in line_ends {
                                        ui.selectable_value(&mut self.input_line_end, line_end.clone(), match line_end.as_str() {
                                            "" => "None".to_string(),
                                            "\n" => "+LF".to_string(),
                                            "\r" => "+CR".to_string(),
                                            "\r\n" => "+CRLF".to_string(),
                                            _ => "".to_string(),
                                        });
                                    }
                                });

                            let response = ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.input_text));

                            if response.lost_focus() && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
                                self.send_message(Message::DataForTransmit);
                                response.request_focus();
                            }
                        });
                    });
                });

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Macros");
                        if ui.button("Set Macros").clicked() {
                            self.macros.set_open(true);
                        }

                        self.macros.show(ctx, ui, self.message_channel.0.clone());

                        for i in 0..16 {
                            if ui.button(format!("M{}{}", i + 1, if i < 10 {" "} else {""})).clicked() {
                                self.send_message(Message::MacroClicked(self.macros.get_macro(i).unwrap().text));
                            }
                        }

                        ui.add_space(ui.available_width());
                    });
                });

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Clear").clicked() {
                            self.output_text.clear();
                        }

                        ui.checkbox(&mut self.hex_output, "Hex output");
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
            });
        });
    }

    fn send_message(&mut self, msg: Message) {
        self.message_channel.0.send(msg).unwrap();
    }

    fn handle_messages(&mut self, ctx: &egui::Context) {
        while let Ok(msg) = self.message_channel.1.try_recv() {
            match msg {
                Message::TryConnect => {
                    if self.serial.try_connect(&self.port, self.serial_settings).is_ok() {

                    }
                },
                Message::TryDisconnect => {
                    if self.serial.try_disconnect().is_ok() {

                    }
                },
                Message::Quit => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                },
                Message::Copy => {
                    if let Ok(mut clipboard) = clipboard::ClipboardContext::new() {
                        clipboard.set_contents(self.output_text.clone()).is_ok();
                    }
                },
                Message::Cut => {
                    if let Ok(mut clipboard) = clipboard::ClipboardContext::new() {
                        clipboard.set_contents(self.output_text.clone()).is_ok();
                        self.output_text.clear();
                    }
                },
                Message::Paste => {
                    if let Ok(mut clipboard) = clipboard::ClipboardContext::new() {
                        self.input_text.push_str(&clipboard.get_contents().unwrap_or_default());
                    }
                },
                Message::DataForTransmit => {
                    let s = &format!("{}{}", self.input_text, self.input_line_end);

                    self.serial.send(s);

                    self.tx_cnt += s.len();
                },
                Message::MacroClicked(msg) => {
                    println!("Macro button clicked with message {}", msg);
                },
                _ => {},
            }
        }
    }

    fn handle_serial_data(&mut self, ctx: &egui::Context) {
        if let Some(s) = self.serial.try_recv() {
            self.rx_cnt += s.len();

            if self.hex_output {
                let mut hex_string = String::new();
                for byte in s.as_bytes() {
                    use std::fmt::Write;
                    write!(hex_string, "{:02X} ", byte).unwrap();
                }
                self.output_text.push_str(&hex_string);
            } else {
                self.output_text.push_str(&s);
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_serial_data(ctx);
        self.handle_messages(ctx);

        self.render_menu_bar(ctx);
        self.render_status_bar(ctx);
        self.render_main_area(ctx);

        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800f32, 800f32]),
            ..Default::default()
    };

    eframe::run_native("Rustcom", native_options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}
