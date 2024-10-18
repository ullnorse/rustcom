use crate::App;

impl App {
    pub fn render_status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label(format!("Device: COM5     Connection: {} @ 8-N-1", self.serial_settings.baud_rate))
        });
    }
}