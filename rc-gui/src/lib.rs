use anyhow::Result;
use eframe::{egui::{self, Style, Visuals, Context, KeyboardShortcut, Modifiers, Key, CentralPanel}, NativeOptions, CreationContext, Frame};
use egui_dock::{Tree, DockArea};
use flume::{unbounded, Sender, Receiver};
use parking_lot::RwLock;
use arboard::Clipboard;

use std::io::Write;
use std::sync::Arc;
use std::fs::OpenOptions;
use std::ops::DerefMut;

mod tabs;
mod widgets;
mod menu_bar;
mod status_bar;
mod modals;

use tabs::{Tab, default_ui};
use widgets::line_end_picker::LineEnd;
use widgets::file_protocol_picker::Protocol;

use rc_core::serial::{SerialConfig, Serial};

#[derive(Clone)]
pub enum Message {
    Connect,
    Disconnect,
    ShowAbout,
    CloseAbout,
    Copy,
    Paste,
    ClearTerminalText,
    ClearLogText,
    Cut,
    SerialDataReceived(String),
    DataForTransmit(String),
    CloseApplication,
    SetDefaultUi,
    RefreshSerialDevices,
    StartRecording,
    StopRecording,
    // Log(Entry),
}

pub struct App {
    channel: (Sender<Message>, Receiver<Message>),
    tree: Arc<RwLock<Tree<Box<dyn Tab>>>>,
    pub serial_config: SerialConfig,
    pub serial: Serial,
    pub terminal_text: String,
    pub log_text: String,
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
            terminal_text: String::new(),
            transmit_text: String::new(),

            device_connected: false,
            current_serial_device: String::new(),
            serial_devices: Serial::available_ports().unwrap_or_default(),

            line_end: LineEnd::default(),
            timestamp: false,
            lock_scrolling: true,

            show_about: false,

            rx_cnt: 0,
            tx_cnt: 0,

            recording_started: false,
            log_file_name: String::new(),

            file_protocol: Protocol::default(),

            log_text: String::new(),
        };

        // Logger::global().set_sender(app.channel.0.clone());

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
                Message::Connect => {
                    if self.serial.start(&self.current_serial_device, self.serial_config.clone()).is_ok() {
                        // info!("{} connected.", self.current_serial_device);
                        self.device_connected = true;
                    } else {
                        // info!("Couldn't connect to {}", self.current_serial_device);
                    }
                },
                Message::Disconnect => {
                    if self.serial.stop().is_ok() {
                        // info!("{} disconnected.", self.current_serial_device);
                        self.device_connected = false;
                    } else {
                        // info!("Couldn't disconnect from {}", self.current_serial_device);
                    }
                },
                Message::DataForTransmit(text) => {
                    if self.device_connected {
                        self.tx_cnt += text.len() as u32;
                        self.serial.send(&text);
                    }
                },
                Message::SerialDataReceived(text) => {
                    if self.timestamp {
                        self.terminal_text.push_str(&chrono::Local::now().format(" %H:%M:%S> ").to_string());
                    }

                    self.rx_cnt += text.len() as u32;
                    self.terminal_text.push_str(&text);

                    if self.recording_started {
                        let mut f = OpenOptions::new()
                            .append(true)
                            .create(true)
                            .truncate(false)
                            .open(self.log_file_name.clone())
                            .unwrap();

                        f.write_all(text.as_bytes()).unwrap();
                    }
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
                Message::CloseApplication => frame.close(),
                Message::SetDefaultUi => *self.tree.write() = default_ui(),
                Message::RefreshSerialDevices => {
                    if let Ok(serial_devices) = Serial::available_ports() {
                        self.serial_devices = serial_devices;
                        self.current_serial_device = self.serial_devices[0].clone();
                    }
                },
                Message::StartRecording => {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_directory(dirs::home_dir().unwrap())
                        .pick_file()
                    {
                        self.log_file_name = path.to_string_lossy().to_string();
                        self.recording_started = true;
                    }
                },
                Message::StopRecording => {
                    self.recording_started = false;
                },
                // Message::Log(entry) => {
                //     entry.format(&mut self.log_text, ctx.style().visuals.dark_mode);
                // },
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
            (Message::ClearTerminalText, KeyboardShortcut::new(Modifiers::CTRL, Key::L)),
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
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
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

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}

pub fn run(device: String, config: SerialConfig) -> Result<()> {
    let min_window_size = Some(egui::Vec2::new(225.0, 225.0));
    let initial_window_size = Some(egui::Vec2::new(1200.0, 800.0));

    let options = NativeOptions {
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
    ).ok();

    Ok(())
}
