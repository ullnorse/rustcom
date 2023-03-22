use super::App;
use egui::{Context, TopBottomPanel};
use eframe::Frame;

impl App {
    pub fn render_status_bar(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label("Not connected");
        });
    }
}