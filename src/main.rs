#![allow(unused)]

mod status_bar;
mod menu_bar;
mod messages;
mod serial;


use crossbeam::channel::{Sender, Receiver, unbounded};
use egui::{Vec2, ViewportBuilder};
use messages::Message;
use serial::{Serial, SerialSettings};

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
    connected: bool,

    input_text: String,
    input_line_end: String,
    output_text: String,

    auto_scroll: bool,
    text_mode: TextMode,
}

impl App {
    fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        let mut app = Self {
            message_channel: unbounded(),
            serial_settings: SerialSettings::default(),
            port: String::new(),
            serial: Serial::new(),
            connected: false,
            input_text: String::new(),
            input_line_end: String::new(),
            output_text: "aleksa".to_string(),
            auto_scroll: false,
            text_mode: TextMode::Ascii,
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
                            if self.connected {
                                if ui.add_sized((70f32, 10f32), egui::Button::new("Disconnect")).clicked() {
                                    self.send_message(Message::TryDisconnect);
                                } 
                            } else if ui.add_sized((70f32, 10f32), egui::Button::new("Connect")).clicked() {
                                self.send_message(Message::TryConnect);
                            }
                        });

                        egui::Grid::new("grid").show(ui, |ui| {
                            ui.checkbox(&mut false, "Auto Dis/Connect");
                            ui.checkbox(&mut false, "Time");

                            ui.end_row();

                            ui.checkbox(&mut false, "Stream log");
                            ui.checkbox(&mut false, "AutoStart Script");

                            ui.end_row();

                            ui.checkbox(&mut false, "CR=LF");
                            ui.checkbox(&mut false, "Stay on top");
                        });

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

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.add_space(10f32);

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.label("Input: ");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            egui::ComboBox::from_id_salt("ComboBox file type").show_ui(ui, |ui| {});
                            ui.button("Send File...");
                            ui.button("Send");

                            
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

                            ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.input_text));
                        });
                    });
                });

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Macros");
                        ui.button("Set Macros");

                        for i in 0..16 {
                            ui.button(format!("M{}", i + 1));
                        }

                        ui.add_space(ui.available_width());
                    });
                });

                
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Clear").clicked() {
                            self.output_text.clear();
                        }

                        ui.checkbox(&mut false, "Hex output");


                    });

                    let selectable_text = |ui: &mut egui::Ui, mut text: &str| {
                        ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut text));
                    };
                    selectable_text(ui, &mut self.output_text);
                });
            });
        });
    }

    fn send_message(&mut self, msg: Message) {
        self.message_channel.0.send(msg).unwrap();
    }

    fn handle_messages(&mut self) {
        while let Ok(msg) = self.message_channel.1.try_recv() {
            match msg {
                Message::TryConnect => {
                    if self.serial.try_connect(&self.port, self.serial_settings).is_ok() {
                        self.connected = true;
                    }
                },
                Message::TryDisconnect => {
                    if self.serial.try_disconnect().is_ok() {
                        self.connected = false;
                    }
                },
                _ => {},
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_messages();

        self.render_menu_bar(ctx);
        self.render_status_bar(ctx);
        self.render_main_area(ctx);
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
