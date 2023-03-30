pub mod widgets;
pub mod menu_bar;
pub mod modals;
pub mod tabs;
mod status_bar;

use crate::serial::Serial;
use crate::serial::serial_config::SerialConfig;
use tabs::{Tab, default_ui};
use widgets::line_end_picker::LineEnd;
use widgets::file_protocol_picker::Protocol;

use egui::{Style, Visuals, Context, CentralPanel, Key, KeyboardShortcut, Modifiers};
use eframe::{NativeOptions, IconData, CreationContext, Frame};
use flume::{unbounded, Sender, Receiver};
use egui_dock::{DockArea, Tree};
use parking_lot::RwLock;
use arboard::Clipboard;
use native_dialog::FileDialog;
use anyhow::Result;

use std::io::Write;
use std::ops::DerefMut;
use std::sync::Arc;
use std::fs::OpenOptions;

#[derive(Clone)]
pub enum Message {
    Connect,
    Disconnect,
    ShowAbout,
    CloseAbout,
    Copy,
    Paste,
    ClearReceiveText,
    Cut,
    SerialDataReceived(String),
    DataForTransmit(String),
    CloseApplication,
    SetDefaultUi,
    RefreshSerialDevices,
    StartRecording,
    StopRecording,
}

pub struct App {
    channel: (Sender<Message>, Receiver<Message>),
    tree: Arc<RwLock<Tree<Box<dyn Tab>>>>,
    pub serial_config: SerialConfig,
    pub serial: Serial,
    pub receive_text: String,
    pub transmit_text: String,

    pub device_connected: bool,
    pub current_serial_device: String,
    pub serial_devices: Vec<String>,

    pub line_end: LineEnd,
    pub timestamp: bool,
    pub lock_scrolling: bool,

    show_about: bool,

    rx_cnt: u32,
    tx_cnt: u32,

    recording_started: bool,
    log_file_name: String,

    pub file_protocol: Protocol,
}

impl App {
    fn new(_cc: &CreationContext, device: String, config: SerialConfig) -> Self {
        let mut app = Self {
            channel: unbounded(),
            tree: Arc::new(RwLock::new(default_ui())),
            serial: Serial::new(),
            serial_config: config,
            receive_text: String::new(),
            transmit_text: String::new(),

            device_connected: false,
            current_serial_device: String::new(),
            serial_devices: Serial::available_ports(),

            line_end: LineEnd::default(),
            timestamp: false,
            lock_scrolling: true,

            show_about: false,

            rx_cnt: 0,
            tx_cnt: 0,

            recording_started: false,
            log_file_name: String::new(),

            file_protocol: Protocol::default(),
        };

        app.current_serial_device = if !device.is_empty() {
            device
        } else if !app.serial_devices.is_empty(){
            app.serial_devices[0].clone()
        } else {
            String::new()
        };

        app
    }

    pub fn do_update(&self, message: Message) {
        self.channel.0.send(message).unwrap();
    }

    fn handle_update(&mut self, _ctx: &Context, frame: &mut Frame) {
        if let Ok(message) = self.channel.1.try_recv() {
            match message {
                Message::Connect => self.serial.start(&self.current_serial_device, self.serial_config.clone()).unwrap(),
                Message::Disconnect => self.serial.stop().unwrap(),
                Message::DataForTransmit(text) => {
                    if self.device_connected {
                        self.tx_cnt += text.len() as u32;
                        self.serial.send(&text);
                    }
                },
                Message::SerialDataReceived(text) => {
                    self.rx_cnt += text.len() as u32;
                    self.receive_text.push_str(&text);

                    if self.timestamp {
                        self.receive_text.push_str(&chrono::Local::now().format(" %H:%M:%S> ").to_string());
                    }

                    if self.recording_started {
                        let mut f = OpenOptions::new()
                            .append(true)
                            .create(true)
                            .truncate(false)
                            .open(self.log_file_name.clone())
                            .unwrap();

                        f.write(text.as_bytes()).unwrap();
                    }
                },
                Message::ShowAbout => self.show_about = true,
                Message::CloseAbout => self.show_about = false,
                Message::Copy => Clipboard::new().unwrap().set_text(self.receive_text.clone()).unwrap(),
                Message::Cut => {
                    Clipboard::new().unwrap().set_text(self.receive_text.clone()).unwrap();
                    self.receive_text.clear();
                },
                Message::Paste => {
                    if let Ok(text) = Clipboard::new().unwrap().get_text() {
                        self.transmit_text.push_str(&text);
                    }
                },
                Message::ClearReceiveText => self.receive_text.clear(),
                Message::CloseApplication => frame.close(),
                Message::SetDefaultUi => *self.tree.write() = default_ui(),
                Message::RefreshSerialDevices => {
                    self.serial_devices = Serial::available_ports();
                    if !self.serial_devices.is_empty() {
                        self.current_serial_device = self.serial_devices[0].clone();
                    }
                },
                Message::StartRecording => {
                    if let Ok(Some(path)) = FileDialog::new()
                        .set_location(dirs::home_dir().unwrap().to_str().unwrap())
                        .show_open_single_file()
                    {
                        self.log_file_name = path.to_string_lossy().to_string();
                        self.recording_started = true;
                    }
                },
                Message::StopRecording => {
                    self.recording_started = false;
                }
            }
        }
    }

    fn handle_serial(&self) {
        if let Some(text) = self.serial.try_recv() {
            self.do_update(Message::SerialDataReceived(text));
        }
    }

    fn handle_repaint(&self, ctx: &Context) {
        if self.device_connected {
            ctx.request_repaint();
        }
    }

    fn handle_keypress(&self, ctx: &Context) {
        let shortcuts = [
            (Message::Copy, KeyboardShortcut::new(Modifiers::CTRL, Key::C)),
            (Message::Cut, KeyboardShortcut::new(Modifiers::CTRL, Key::X)),
            (Message::Paste, KeyboardShortcut::new(Modifiers::CTRL, Key::V)),
            (Message::ClearReceiveText, KeyboardShortcut::new(Modifiers::CTRL, Key::L)),
        ];

        for (message, shortcut) in &shortcuts {
            if ctx.input_mut(|i| i.consume_shortcut(shortcut)) {
                self.do_update(message.clone());
                break;
            }
        }

        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Enter)) {
            let mut s = self.transmit_text.clone();
            s.push_str(self.line_end.into());
            self.do_update(Message::DataForTransmit(s));
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.handle_update(ctx, frame);
        self.render_menu_bar(ctx);
        self.render_status_bar(ctx, frame);

        self.handle_serial();

        self.handle_repaint(ctx);
        self.handle_keypress(ctx);

        self.render_about(ctx);

        CentralPanel::default().show(ctx, |ui| {
            DockArea::new(self.tree.clone().write().deref_mut()).show_inside(ui, self);
        });
    }
}

pub fn run(device: String, config: SerialConfig) -> Result<(), eframe::Error> {
    let icon_data = Some(IconData {
        height: 256,
        width: 256,
        rgba: image::load_from_memory(include_bytes!("../assets/icon.png"))
            .unwrap()
            .to_rgba8()
            .into_vec(),
    });

    let min_window_size = Some(egui::Vec2::new(225.0, 225.0));
    let initial_window_size = Some(egui::Vec2::new(1200.0, 800.0));

    let options = NativeOptions {
        icon_data,
        min_window_size,
        initial_window_size,
        ..Default::default()
    };

    let style = Style {
        visuals: Visuals::light(),
        ..Style::default()
    };

    eframe::run_native("rustcom", options, Box::new(|cc| {
            cc.egui_ctx.set_style(style);
            Box::new(App::new(cc, device, config))
        })
    )
}
