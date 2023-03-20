use super::{App, Message};
use egui::{Context, TopBottomPanel, Ui};
use eframe::Frame;

impl App {
    pub fn render_menu(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.style_mut().visuals.button_frame = true;
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| self.file_menu(ui));
                ui.menu_button("Edit", |ui| self.edit_menu(ui));
                ui.menu_button("Window", |ui| self.window_menu(ui));
                ui.menu_button("Help", |ui| self.help_menu(ui));
            });
        });
    }

    pub fn file_menu(&self, ui: &mut Ui) {
        ui.button("button31");
        ui.button("button32");
    }

    pub fn edit_menu(&self, ui: &mut Ui) {
        ui.button("button41");
        ui.button("button42");
    }

    pub fn window_menu(&self, ui: &mut Ui) {
        ui.button("button21");
        ui.button("button22");
    }

    pub fn help_menu(&self, ui: &mut Ui) {
        ui.button("button11");
        if ui.button("About").clicked() {
            self.do_update(Message::ShowAbout);
        }
    }
}
