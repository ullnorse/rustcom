use eframe::egui::{Context, TopBottomPanel, Ui};
use log::error;

use crate::app::App;

fn create_menu_item(label: &str, ui: &mut Ui, mut callback: impl FnMut()) {
    if ui.button(format!("{:<30}", label)).clicked() {
        callback();
        ui.close();
    }
}

fn file_menu(app: &mut App, ui: &mut Ui, ctx: &Context) {
    create_menu_item("Quit", ui, || app.quit(ctx));
}

fn edit_menu(app: &mut App, ui: &mut Ui) {
    create_menu_item("Cut", ui, || {
        if let Err(e) = app.cut() {
            error!("Cut operation failed: {e}");
        }
    });

    create_menu_item("Copy", ui, || {
        if let Err(e) = app.copy() {
            error!("Copy operation failed: {e}");
        }
    });

    create_menu_item("Paste", ui, || {
        if let Err(e) = app.paste() {
            error!("Paste operation failed: {e}");
        }
    });

    ui.separator();

    create_menu_item("Clear", ui, || app.output_text.clear());
}

fn help_menu(app: &mut App, ui: &mut Ui) {
    create_menu_item("Show About", ui, || app.about_window_open = true);
    create_menu_item("Show Log", ui, || app.logger_window_open = true);
}

impl App {
    pub fn render_menu_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.style_mut().visuals.button_frame = false;

            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| file_menu(self, ui, ctx));
                ui.menu_button("Edit", |ui| edit_menu(self, ui));
                ui.menu_button("Help", |ui| help_menu(self, ui));
            });
        });
    }
}
