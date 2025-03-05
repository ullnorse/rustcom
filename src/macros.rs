use crossbeam::channel::Sender;
use crate::messages::Message;
use std::{io::Write, path::Path, sync::{atomic::{AtomicBool, Ordering}, Arc}};
use std::thread;
use std::time::Duration;
use anyhow::Result;

#[derive(Clone)]
pub struct Macro {
    pub text: String,
    pub delay: u32,
    pub repeat: bool,
    pub active: Arc<AtomicBool>,
}

impl Default for Macro {
    fn default() -> Self {
        Self {
            text: String::new(),
            delay: 1000,
            repeat: false,
            active: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub struct Macros {
    macros: [Macro; 16],
    config_file: String,
}

impl Default for Macros {
    fn default() -> Self {
        Self {
            macros: core::array::from_fn(|_| Macro::default()),
            config_file: String::new(),
        }
    }
}

impl Macros {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_window(&mut self, open: &mut bool, ctx: &egui::Context, sender: Sender<Message>) {
        let mut repeat_changes = Vec::new();

        egui::Window::new("Macros")
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let button_size = egui::vec2(60.0, 30.0);

                        if ui.add_sized(button_size, egui::Button::new("Load")).clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("Open")
                                .set_directory(directories::BaseDirs::new().unwrap().home_dir())
                                .pick_file() {
                                    self.read_config_from_file(path.as_path());
                                    self.config_file = path.into_os_string().into_string().unwrap();
                                }
                        }

                        if ui.add_sized(button_size, egui::Button::new("Save")).clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("Save As")
                                .set_directory(directories::BaseDirs::new().unwrap().home_dir())
                                .save_file() {
                                    self.save_config_to_file(path.as_path()).unwrap();
                                }
                        }

                        ui.label(&self.config_file);
                    });

                    for i in 0..16 {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            let repeat = &mut self.macros[i].repeat;
                            if ui.checkbox(repeat, "").changed() {
                                repeat_changes.push(i);
                            }

                            spinbox(ui, &mut self.macros[i].delay, 0, u32::MAX, 10);

                            if ui.add_sized(egui::vec2(50.0, 20.0), egui::Button::new(format!("M{}", i + 1))).clicked() {
                                sender.send(Message::MacroClicked(self.macros[i].text.clone())).unwrap();
                            }

                            ui.text_edit_singleline(&mut self.macros[i].text);
                        });
                    }
                });
            });

        for i in repeat_changes {
            let sender_clone = sender.clone();
            self.handle_macro_repeat(i, sender_clone);
        }
    }

    pub fn render_ui(&mut self, open: bool, window_open: &mut bool, ui: &mut egui::Ui, sender: Sender<Message>) {
        if open {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Macros");
                    if ui.button("Set Macros").clicked() {
                        *window_open = true;
                    }

                    for i in 0..self.macros.len() {
                        if ui.button(format!("M{}{}", i + 1, if i < 10 {" "} else {""})).clicked() {
                            sender.send(Message::MacroClicked(self.macros[i].text.clone())).unwrap();
                        }
                    }

                    ui.add_space(ui.available_width());
                });
            });
        }
    }

    fn save_config_to_file(&mut self, path: &Path) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let mut config = String::new();
        for i in 0..self.macros.len() {
            config.push_str(&format!("M{}\n", i + 1));
            config.push_str(&self.macros[i].text);
            config.push('\n');
        }

        file.write_all(config.as_bytes())?;

        Ok(())
    }

    fn read_config_from_file(&mut self, path: &Path) {
        if let Ok(config) = std::fs::read_to_string(path) {
            for (i, line) in config.lines().skip(1).step_by(2).enumerate() {
                if i >= self.macros.len() {
                    break;
                }

                self.macros[i].text = line.trim().to_string();
            }
        }
    }

    fn handle_macro_repeat(&mut self, index: usize, sender: Sender<Message>) {
        let mac = &mut self.macros[index];

        if mac.repeat && !mac.active.load(Ordering::SeqCst) {
            let delay = mac.delay;
            let active_flag = mac.active.clone();
            active_flag.store(true, Ordering::SeqCst);

            let text = mac.text.clone();

            thread::spawn(move || {
                while active_flag.load(Ordering::SeqCst) {
                    sender.send(Message::MacroClicked(text.clone())).unwrap();
                    thread::sleep(Duration::from_millis(delay as u64));
                }
            });
        } else if !mac.repeat && mac.active.load(Ordering::SeqCst) {
            mac.active.store(false, Ordering::SeqCst);
        }
    }
}

fn spinbox(ui: &mut egui::Ui, value: &mut u32, min: u32, max: u32, step: u32) {
    ui.horizontal(|ui| {
        if ui.add(egui::Button::new("+")).clicked() {
            *value = (*value + step).min(max);
        }

        ui.add(
            egui::DragValue::new(value)
                .range(0..=u32::MAX)
                .speed(step as f64)
        );

        if ui.add(egui::Button::new("-")).clicked() {
            *value = value.saturating_sub(step).max(min);
        }
    });
}
