mod menu_bar;
mod status_bar;
mod tabs;
mod widgets;
mod modals;

use anyhow::Result;
use base::serial::{SerialConfig, Serial};
use eframe::egui::{Key, KeyboardShortcut, Modifiers, Vec2, ViewportBuilder, Visuals};
use egui_dock::{DockArea, DockState, Style};
use flume::{unbounded, Receiver, Sender};
use widgets::line_end_picker::LineEnd;
use base::messages::{self, Message};
use crate::tabs::{Tab, default_ui};
use arboard::Clipboard;
use base::logger::{self, Logger, LOGGER};
use log::info;

use std::sync::RwLock;

use std::rc::Rc;

struct App {
    channel: (Sender<Message>, Receiver<Message>),
    tree: Rc<RwLock<DockState<Box<dyn Tab>>>>,

    current_serial_device: String,
    serial_devices: Vec<String>,
    serial_config: SerialConfig,
    device_connected: bool,

    serial: Serial,

    recording_started: bool,
    timestamp: bool,
    lock_scrolling: bool,
    show_about: bool,

    transmit_text: String,
    terminal_text: String,
    log_text: String,
    line_end: LineEnd,

    tx_cnt: u32,
    rx_cnt: u32,
}

impl App {
    fn new(_cc: &eframe::CreationContext, device: String, serial_config: SerialConfig) -> Self {
        let mut app = Self {
            channel: unbounded(),
            tree: Rc::new(RwLock::new(default_ui())),

            current_serial_device: String::new(),
            serial_devices: Serial::available_ports().unwrap(),
            serial_config: serial_config.clone(),
            device_connected: false,

            serial: Serial::new(),

            recording_started: false,
            timestamp: false,
            lock_scrolling: true,
            show_about: false,

            transmit_text: String::new(),
            terminal_text: String::new(),
            log_text: String::new(),
            line_end: LineEnd::default(),

            tx_cnt: 0,
            rx_cnt: 0,
        };

        Logger::global().set_sender(app.channel.0.clone());

        app.current_serial_device = if !device.is_empty() {
            device
        } else if !app.serial_devices.is_empty() {
            app.serial_devices[0].clone()
        } else {
            String::new()
        };

        app
    }

    fn render_main_area(&mut self, ctx: &eframe::egui::Context) {
        DockArea::new(self.tree.clone().write().as_deref_mut().unwrap())
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, self)
    }

    fn handle_message(&mut self, ctx: &eframe::egui::Context) {
        if let Some(message) = messages::try_receive() {
            match message {
                Message::Connect => {
                    if self.serial.start(&self.current_serial_device, self.serial_config.clone()).is_ok() {
                        info!("{} connected.", self.current_serial_device);
                        self.device_connected = true;
                    } else {
                        info!("Couldn't connect to {}", self.current_serial_device);
                    }
                },
                Message::Disconnect => {
                    if self.serial.stop().is_ok() {
                        info!("{} disconnected.", self.current_serial_device);
                        self.device_connected = false;
                    } else {
                        info!("Couldn't disconnect from {}", self.current_serial_device);
                    }
                },
                Message::DataForTransmit(text) => {
                    if self.device_connected {
                        self.tx_cnt += text.len() as u32;
                        self.serial.send(&text);
                    }
                },
                Message::DataReceived(text) => {
                    if self.timestamp {
                        self.terminal_text.push_str(&chrono::Local::now().format(" %H:%M:%S> ").to_string());
                    }

                    self.rx_cnt += text.len() as u32;
                    self.terminal_text.push_str(&text);

                    // if self.recording_started {
                    //     let mut f = OpenOptions::new()
                    //         .append(true)
                    //         .create(true)
                    //         .truncate(false)
                    //         .open(self.log_file_name.clone())
                    //         .unwrap();

                    //     f.write_all(text.as_bytes()).unwrap();
                    // }
                },
                Message::ShowAbout => self.show_about = true,
                Message::CloseAbout => self.show_about = false,
                Message::Copy => Clipboard::new().unwrap().set_text(self.terminal_text.clone()).unwrap(),
                Message::Cut => {
                    Clipboard::new().unwrap().set_text(self.terminal_text.clone()).unwrap();
                    self.terminal_text.clear();
                },
                Message::Paste => {
                    if let Ok(text) = Clipboard::new().unwrap().get_text() {
                        self.transmit_text.push_str(&text);
                    }
                },
                Message::ClearTerminalText => self.terminal_text.clear(),
                Message::ClearLogText => self.log_text.clear(),
                Message::CloseApplication => ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close),
                Message::SetDefaultUi => *self.tree.write().as_deref_mut().unwrap() = default_ui(),
                Message::Log(entry) => {
                    entry.format(&mut self.log_text);
                },
                _ => {}
            }
        }
    }

    fn handle_serial(&self) {
        if let Some(text) = self.serial.try_recv() {
            messages::send(Message::DataReceived(text));
        }
    }

    fn handle_repaint(&self, ctx: &eframe::egui::Context) {
        if self.device_connected {
            ctx.request_repaint();
        }
    }

    fn handle_keypress(&self, ctx: &eframe::egui::Context) {
        let shortcuts = [
            (Message::Copy, KeyboardShortcut::new(Modifiers::CTRL, Key::C)),
            (Message::Cut, KeyboardShortcut::new(Modifiers::CTRL, Key::X)),
            (Message::Paste, KeyboardShortcut::new(Modifiers::CTRL, Key::V)),
            (Message::ClearTerminalText, KeyboardShortcut::new(Modifiers::CTRL, Key::L)),
        ];

        for (message, shortcut) in &shortcuts {
            if ctx.input_mut(|i| i.consume_shortcut(shortcut)) {
                messages::send(message.clone());
                break;
            }
        }

        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Enter)) {
            let mut s = self.transmit_text.clone();
            s.push_str(self.line_end.into());
            messages::send(Message::DataForTransmit(s));
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.handle_message(ctx);
        self.handle_serial();
        self.handle_repaint(ctx);
        self.handle_keypress(ctx);

        self.render_menu_bar(ctx);
        self.render_status_bar(ctx);
        self.render_main_area(ctx);
        self.render_about(ctx);
    }
}

pub fn run(device: String, config: SerialConfig) -> Result<()> {
    let logger = Logger::default();
    LOGGER.set(logger).unwrap();
    logger::init();

    messages::init();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(Vec2::new(1200.0, 800.0)),
        ..Default::default()
    };

    let style = egui_dock::egui::Style {
        visuals: Visuals::light(),
        ..Default::default()
    };

    eframe::run_native("rustcom", native_options, Box::new(|cc| {
            cc.egui_ctx.set_style(style);
            Box::new(App::new(cc, device, config))
        })
    ).ok();

    Ok(())
}