#![allow(unused)]

mod status_bar;
mod messages;


use crossbeam::channel::{Sender, Receiver, unbounded};
use egui::ViewportBuilder;
use messages::Message;

use serialport5::{DataBits, Parity, StopBits, FlowControl};

#[derive(PartialEq)]
enum TextMode {
    Hex,
    Ascii,
}

struct App {
    baud_rate: i32,
    data_bits: DataBits,
    parity: Parity,
    stop_bits: StopBits,
    flow_control: FlowControl,

    input_text: String,
    input_add_cr: bool,
    output_text: String,

    auto_scroll: bool,
    text_mode: TextMode,
}

impl App {
    fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        Self {
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One,
            flow_control: FlowControl::None,
            input_text: String::new(),
            input_add_cr: false,
            output_text: "aleksa".to_string(),
            auto_scroll: false,
            text_mode: TextMode::Ascii,
        }
    }

    fn render_main_area(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if ui.button("Connect").clicked() {

                        }

                        if ui.button("Rescan").clicked() {
                            
                        }

                        if ui.button("Help").clicked() {
                            
                        }

                        if ui.button("About").clicked() {
                            
                        }

                        if ui.button("Quit").clicked() {
                            
                        }
                    });

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            egui::ComboBox::from_id_salt("COM Port").show_ui(ui, |ui| {

                            });

                            ui.button("Refresh");
                            ui.add_space(ui.available_height());
                        });
                    });

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Baud rate");

                            egui::Grid::new("grid_baud_rate").show(ui, |ui| {
                                ui.radio_value(&mut self.baud_rate, 600, "600");
                                ui.radio_value(&mut self.baud_rate, 14400, "14400");
                                ui.radio_value(&mut self.baud_rate, 57600, "57600");
        
                                ui.end_row();
        
                                ui.radio_value(&mut self.baud_rate, 1200, "1200");
                                ui.radio_value(&mut self.baud_rate, 19200, "19200");
                                ui.radio_value(&mut self.baud_rate, 115200, "115200");

                                ui.end_row();

                                ui.radio_value(&mut self.baud_rate, 2400, "2400");
                                ui.radio_value(&mut self.baud_rate, 28800, "28800");
                                ui.radio_value(&mut self.baud_rate, 128000, "128000");

                                ui.end_row();

                                ui.radio_value(&mut self.baud_rate, 4800, "4800");
                                ui.radio_value(&mut self.baud_rate, 38400, "38400");
                                ui.radio_value(&mut self.baud_rate, 256000, "256000");

                                ui.end_row();

                                ui.radio_value(&mut self.baud_rate, 9600, "9600");
                                ui.radio_value(&mut self.baud_rate, 56000, "56000");
                                ui.radio_value(&mut self.baud_rate, 1000000, "1000000");
                            });
                        });
                    });

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Data bits");

                            ui.radio_value(&mut self.data_bits, DataBits::Five, "5");
                            ui.radio_value(&mut self.data_bits, DataBits::Six, "6");
                            ui.radio_value(&mut self.data_bits, DataBits::Seven, "7");
                            ui.radio_value(&mut self.data_bits, DataBits::Eight, "8");

                            ui.add_space(ui.available_height());
                        });
                    });

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Parity");

                            ui.radio_value(&mut self.parity, Parity::None, "None");
                            ui.radio_value(&mut self.parity, Parity::Odd, "Odd");
                            ui.radio_value(&mut self.parity, Parity::Even, "Even");

                            ui.add_space(ui.available_height());
                        });
                    });

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Stop bits");

                            ui.radio_value(&mut self.stop_bits, StopBits::One, "1");
                            ui.radio_value(&mut self.stop_bits, StopBits::Two, "2");

                            ui.add_space(ui.available_height());
                        });
                    });

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Flow control");

                            ui.radio_value(&mut self.flow_control, FlowControl::None, "None");
                            ui.radio_value(&mut self.flow_control, FlowControl::Hardware, "Hardware");
                            ui.radio_value(&mut self.flow_control, FlowControl::Software, "Software");

                            ui.add_space(ui.available_height());
                        });
                    });
                });

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        egui::Grid::new("grid1").show(ui, |ui| {
                            ui.checkbox(&mut false, "Auto Dis/Connect");
                            ui.checkbox(&mut false, "Time");
                            ui.checkbox(&mut false, "Stream log");

                            ui.end_row();

                            ui.checkbox(&mut false, "AutoStart Script");
                            ui.checkbox(&mut false, "CR=LF");
                            ui.checkbox(&mut false, "Stay on top");
                        });

                        ui.add_space(ui.available_width());
                    });
                });

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.button("Clear");
                        ui.checkbox(&mut self.auto_scroll, "Auto Scroll");
                        ui.button("Reset Cnt");
                        ui.text_edit_singleline(&mut "13");
                        ui.label("Cnt = 13");

                        ui.vertical(|ui| {
                            ui.radio_value(&mut self.text_mode, TextMode::Hex, "HEX");
                            ui.radio_value(&mut self.text_mode, TextMode::Ascii, "ASCII");
                        });

                        ui.vertical(|ui| {
                            ui.checkbox(&mut false, "Log date stamp");

                            ui.horizontal(|ui| {
                                ui.button("Start Log");
                                ui.button("Stop Log");
                            });
                        });

                        ui.button("Req/Resp");
                        

                        ui.add_space(ui.available_width());
                    });
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.add_space(10f32);

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            ui.button("Send");
                            ui.checkbox(&mut self.input_add_cr, "+CR");

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
                            ui.label("Transmit");
                            ui.button("Clear");
                            ui.button("Send File");
                            ui.add_space(ui.available_width());
                    });
                });



                let selectable_text = |ui: &mut egui::Ui, mut text: &str| {
                    ui.group(|ui| {
                        ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut text));
                    });
                };

                selectable_text(ui, &mut self.output_text);
            });
        });
    }

    fn handle_messages(&mut self) {

    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_messages();

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
