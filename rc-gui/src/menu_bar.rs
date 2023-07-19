use super::{App, Message};
use eframe::egui::{Context, TopBottomPanel, Ui};

impl App {
    fn create_menu_item(&self, ui: &mut Ui, label: &str, message: Message, shortcut: Option<&str>) {
        if ui.button(format!("{:<30}{}", label, shortcut.unwrap_or_default())).clicked() {
            self.do_update(message);
            ui.close_menu();
        }
    }

    pub fn render_menu_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.style_mut().visuals.button_frame = false;
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| self.file_menu(ui));
                ui.menu_button("Edit", |ui| self.edit_menu(ui));
                ui.menu_button("Window", |ui| self.window_menu(ui));
                ui.menu_button("Help", |ui| self.help_menu(ui));
            });
        });
    }

    pub fn file_menu(&self, ui: &mut Ui) {
        self.create_menu_item(ui, "Quit", Message::CloseApplication, None);
    }

    pub fn edit_menu(&self, ui: &mut Ui) {
        self.create_menu_item(ui, "Cut", Message::Cut, Some("  Ctrl+X"));
        self.create_menu_item(ui, "Copy", Message::Copy, Some("Ctrl+C"));
        self.create_menu_item(ui, "Paste", Message::Paste, Some("Ctrl+V"));
        ui.separator();
        self.create_menu_item(ui, "Clear", Message::ClearTerminalText, Some("Ctrl+L"));
    }

    pub fn window_menu(&self, ui: &mut Ui) {
        self.create_menu_item(ui, "Reset", Message::SetDefaultUi, None);
    }

    pub fn help_menu(&self, ui: &mut Ui) {
        self.create_menu_item(ui, "About", Message::ShowAbout, None);
    }
}
