use super::App;
use egui::{Context, TopBottomPanel, TextBuffer, widgets::global_dark_light_mode_switch};
use eframe::Frame;

impl App {
    pub fn render_status_bar(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.label(format!("{} {} | {}, {}{}{} flow control: {}           TX: {} / RX: {}      {}",
                    self.current_serial_device,
                    if self.device_connected {
                        "OPENED"
                    } else {
                        "CLOSED"
                    },
                    self.serial_config.baudrate,

                    format!("{:?}", self.serial_config.char_size).char_range(4..5),
                    format!("{:?}", self.serial_config.parity).char_range(0..1),
                    format!("{:?}", self.serial_config.stop_bits).char_range(0..1),
                    format_args!("{:?}", self.serial_config.flow_control),

                    self.tx_cnt,
                    self.rx_cnt,

                    if self.recording_started {
                        format!("Logging to: {}", self.log_file_name)
                    } else {
                        String::new()
                    }
                ));
            });
        });
    }
}
