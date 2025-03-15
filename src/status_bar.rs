#![allow(clippy::format_in_format_args)]

use crate::app::App;
use egui::TextBuffer;

impl App {
    pub fn render_status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::global_theme_preference_switch(ui);

                ui.label(format!(
                    "{} {} | {}, {}-{}-{} flow control: {:?}           TX: {} | RX: {}      {}",
                    self.port,
                    if self.serial.is_some() {
                        "OPENED"
                    } else {
                        "CLOSED"
                    },
                    self.serial_settings.baud_rate,
                    format!("{}", self.serial_settings.data_bits),
                    format!("{:?}", self.serial_settings.parity).char_range(0..1),
                    format!("{}", self.serial_settings.stop_bits),
                    self.serial_settings.flow_control,
                    self.tx_cnt,
                    self.rx_cnt,
                    //self.recording_started
                    if false {
                        format!("Logging to: {}", "") // self.log_file_name
                    } else {
                        String::new()
                    }
                ));
            });
        });
    }
}
