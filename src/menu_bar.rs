use crate::messages::Message;
use crate::app::App;

fn create_menu_item<F>(ui: &mut egui::Ui, label: &str, shortcut: Option<&str>, mut callback: F)
where
    F: FnMut()
{
    if ui.button(format!("{:<30}{}", label, shortcut.unwrap_or_default())).clicked() {
        callback();
        ui.close_menu();
    }
}

fn file_menu(app: &mut App, ui: &mut egui::Ui) {
    create_menu_item(ui, "Quit", None, || app.send_message(Message::Quit));
}

fn edit_menu(app: &mut App, ui: &mut egui::Ui) {
    create_menu_item(ui, "Cut", Some("  Ctrl+X"), || app.send_message(Message::Cut));
    create_menu_item(ui, "Copy", Some("Ctrl+C"), || app.send_message(Message::Copy));
    create_menu_item(ui, "Paste", Some("Ctrl+V"), || app.send_message(Message::Paste));

    ui.separator();

    create_menu_item(ui, "Clear", Some("Ctrl+L"), || app.send_message(Message::ClearReceiveText));
}

fn help_menu(app: &mut App, ui: &mut egui::Ui) {
    create_menu_item(ui, "About", None, || app.send_message(Message::ShowAbout));
    create_menu_item(ui, "Log", None, || app.send_message(Message::ShowLog));
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