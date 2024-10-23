use crossbeam::channel::Sender;
use crate::messages::Message;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

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
    window_open: bool,
}

impl Default for Macros {
    fn default() -> Self {
        Self {
            macros: core::array::from_fn(|_| Macro::default()),
            window_open: false,
        }
    }
}

impl Macros {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, ctx: &egui::Context, sender: Sender<Message>) {
        let mut repeat_changes = Vec::new();

        egui::Window::new("Macros")
            .open(&mut self.window_open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let button_size = egui::vec2(60.0, 30.0);
                        ui.add_sized(button_size, egui::Button::new("Load"));
                        ui.add_sized(button_size, egui::Button::new("Save"));
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

    pub fn set_open(&mut self, state: bool) {
        self.window_open = state;
    }

    pub fn get_macro(&self, index: usize) -> Option<Macro> {
        self.macros.get(index).cloned()
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
