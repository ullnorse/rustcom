use super::App;
use egui::{Context, TopBottomPanel, Ui};
use eframe::Frame;

impl App {
    pub fn render_status_bar(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label("Not connected");
        });
    }
}