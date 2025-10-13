use crate::app::App;
use eframe::egui::{Context, TextBuffer, TopBottomPanel, global_theme_preference_switch};

impl App {
    pub fn render_status_bar(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_theme_preference_switch(ui);

                ui.label(format!(
                    "{} {} | {}, {}-{}-{} flow control: {:?}           TX: {} | RX: {}",
                    self.serial_settings.port,
                    if self.serial.is_some() {
                        "OPENED"
                    } else {
                        "CLOSED"
                    },
                    self.serial_settings.baud_rate,
                    format_args!("{}", self.serial_settings.data_bits),
                    format!("{:?}", self.serial_settings.parity).char_range(0..1),
                    format_args!("{}", self.serial_settings.stop_bits),
                    self.serial_settings.flow_control,
                    self.tx_cnt,
                    self.rx_cnt,
                ));
            });
        });
    }
}
