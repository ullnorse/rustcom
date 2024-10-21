use std::sync::{Mutex, OnceLock};
use log::Level;

pub static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn init() {
    log::set_logger(Logger::global()).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

#[derive(Debug)]
pub struct Logger {
    log: Mutex<String>,
    window_open: Mutex<bool>,
    log_level: Mutex<Level>,
}

impl Logger {
    pub fn global() -> &'static Logger {
        LOGGER.get().expect("logger is not initialized")
    }

    pub fn new() -> Self {
        Self {
            log: Mutex::new(String::new()),
            window_open: Mutex::new(false),
            log_level: Mutex::new(Level::Error),
        }
    }

    pub fn show(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Window::new("Log")
            .resizable(false)
            .open(&mut self.window_open.lock().unwrap())
            .constrain(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Clear").clicked() {
                        self.log.lock().unwrap().clear();
                    }

                    let log_levels = [Level::Error, Level::Warn, Level::Info, Level::Debug, Level::Trace];
                    let mut log_level = Level::Error;

                    egui::ComboBox::from_label("Log level")
                        .selected_text(format!("{:?}", *self.log_level.lock().unwrap()))
                        .show_ui(ui, |ui| {
                            for level in log_levels {
                                ui.selectable_value(&mut *self.log_level.lock().unwrap(), level, level.as_str());
                            }
                    });
                });

                let selectable_text = |ui: &mut egui::Ui, mut text: &str| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut text));
                    });
                };

                ui.group(|ui| {
                    egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                selectable_text(ui, self.log.lock().unwrap().as_str());
                            });
                });
            });
    }

    pub fn set_open(&self, state: bool) {
        *self.window_open.lock().unwrap() = state;
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= *self.log_level.lock().unwrap()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.log.lock().unwrap().push_str(&format!("{} - {}\n", record.level(), record.args()));
        }
    }

    fn flush(&self) {
        todo!()
    }
}