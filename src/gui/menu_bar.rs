use super::{App, Message};
use egui::{Context, TopBottomPanel, Ui};
use eframe::Frame;
use super::default_ui;

impl App {
    pub fn render_menu(&mut self, ctx: &Context, _frame: &mut Frame) {
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
        if ui.button("Open Settings").clicked() {

        }

        if ui.button("Open Recent").clicked() {

        }

        ui.separator();

        if ui.button("Save Settings").clicked() {

        }
        if ui.button("Save Settings As").clicked() {

        }

        ui.separator();

        if ui.button("Quit").clicked() {

        }
    }

    pub fn edit_menu(&self, ui: &mut Ui) {
        if ui.button("Cut                              Ctrl+X").clicked() {
            self.do_update(Message::Cut);
            ui.close_menu();
        }

        if ui.button("Copy                           Ctrl+C").clicked() {
            self.do_update(Message::Copy);
            ui.close_menu();
        }

        if ui.button("Paste                          Ctrl+V").clicked() {
            self.do_update(Message::Paste);
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Clear                           Ctrl+L").clicked() {
            self.do_update(Message::ClearReceiveText);
            ui.close_menu();
        }
    }

    pub fn window_menu(&self, ui: &mut Ui) {
        if ui.button("Reset").clicked() {
            ui.close_menu();
            *self.tree.write() = default_ui();
        }
    }

    pub fn help_menu(&self, ui: &mut Ui) {
        if ui.button("About").clicked() {
            self.do_update(Message::ShowAbout);
        }
    }
}
