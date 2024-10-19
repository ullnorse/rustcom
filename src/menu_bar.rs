use crate::{messages::Message, App};

fn create_menu_item(app: &mut App, ui: &mut egui::Ui, label: &str, message: Message, shortcut: Option<&str>) {
    if ui.button(format!("{:<30}{}", label, shortcut.unwrap_or_default())).clicked() {
        app.send_message(message);
        ui.close_menu();
    }
}

fn file_menu(app: &mut App, ui: &mut egui::Ui) {
    create_menu_item(app, ui, "Quit", Message::Quit, None);
}

fn edit_menu(app: &mut App, ui: &mut egui::Ui) {
    create_menu_item(app, ui, "Cut", Message::Cut, Some("  Ctrl+X"));
    create_menu_item(app, ui, "Copy", Message::Copy, Some("Ctrl+C"));
    create_menu_item(app, ui, "Paste", Message::Paste, Some("Ctrl+V"));

    ui.separator();

    create_menu_item(app, ui, "Clear", Message::ClearReceiveText, Some("Ctrl+L"));
}

fn help_menu(app: &mut App, ui: &mut egui::Ui) {
    create_menu_item(app, ui, "About", Message::ShowAbout, None);
}

impl App {
    pub fn render_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.style_mut().visuals.button_frame = false;

            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| file_menu(self, ui));
                ui.menu_button("Edit", |ui| edit_menu(self, ui));
                ui.menu_button("Help", |ui| help_menu(self, ui));
            });
        });
    }
}