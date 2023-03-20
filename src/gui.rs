pub mod port_settings;
pub mod line_end_picker;
pub mod menu_bar;
pub mod modals;
mod status_bar;

use super::serial::SerialConfig;
use super::tabs::{Tabs, default_ui};

use egui::{Style, Visuals, Context, LayerId, Id, Ui, CentralPanel};
use eframe::{NativeOptions, IconData, CreationContext, Frame};
use flume::{unbounded, Sender, Receiver};
use egui_dock::{DockArea, Tree};
use parking_lot::RwLock;

use std::ops::DerefMut;
use std::sync::Arc;

pub enum Message {
    ShowAbout,
    CloseAbout,
    HelloWorld,
}

pub struct App {
    channel: (Sender<Message>, Receiver<Message>),
    tree: Arc<RwLock<Tree<Tabs>>>,
    pub device: String,
    pub serial_config: SerialConfig,
    pub terminal_text: String,

    show_about: bool,
}

impl App {
    fn new(cc: &CreationContext, device: String, config: SerialConfig) -> Self {
        Self {
            channel: unbounded(),
            tree: Arc::new(RwLock::new(default_ui())),
            device,
            serial_config: config,
            terminal_text: String::new(),
            show_about: false,
        }
    }

    fn do_update(&self, message: Message) {
        self.channel.0.send(message).unwrap();
    }

    fn handle_update(&mut self, ctx: &Context, frame: &mut Frame) {
        if let Ok(message) = self.channel.1.try_recv() {
            match message {
                Message::HelloWorld => println!("Hello World"),
                Message::ShowAbout => self.show_about = true,
                Message::CloseAbout => self.show_about = false,
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.handle_update(ctx, frame);
        self.render_menu(ctx, frame);
        self.render_status_bar(ctx, frame);

        self.render_about(ctx);

        CentralPanel::default().show(ctx, |ui| {
            DockArea::new(self.tree.clone().write().deref_mut()).show_inside(ui, self);
        });
    }
}

pub fn run(device: String, config: SerialConfig) {
    println!("Started GUI app");

    eframe::run_native(
        "rustcom",
        NativeOptions {
            icon_data: Some(IconData {
                height: 256,
                width:  256,
                rgba:   image::load_from_memory(include_bytes!("../assets/icon.png"))
                    .unwrap()
                    .to_rgba8()
                    .into_vec(),
            }),
            min_window_size: Some(egui::Vec2::new(225.0, 225.0)),
            initial_window_size: Some(egui::Vec2::new(1200.0, 800.0)),
            ..Default::default()
        },
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::light(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Box::new(App::new(cc, device, config))
        }),
    ).ok();
}