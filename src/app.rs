use crate::logger::Logger;
use crate::macros::Macros;
use crate::serial::{SerialMainState, SerialMsg, SerialSettings};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::fmt::Write;
use std::time::Duration;
use thiserror::Error;

use log::{error, info};

use crate::ui::windows::{about_window, logger_window, macros_window};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Other")]
    Other,
}

pub struct App {
    pub ctx: egui::Context,
    pub serial_settings: SerialSettings,
    pub port: String,
    pub available_ports: Vec<String>,
    pub serial: Option<SerialMainState>,

    pub input_text: String,
    pub input_line_end: String,
    pub output_text: String,

    pub auto_scroll: bool,
    pub hex_output: bool,

    pub tx_cnt: usize,
    pub rx_cnt: usize,

    pub macros: Macros,
    pub macros_ui_open: bool,
    pub logger_window_open: bool,
    pub about_window_open: bool,
    pub macros_window_open: bool,
}

impl App {
    pub fn new(
        port: String,
        serial_settings: SerialSettings,
        cc: &eframe::CreationContext,
    ) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Light);

        let mut app = Self {
            ctx: cc.egui_ctx.clone(),
            serial_settings,
            port,
            available_ports: SerialMainState::available_ports(),
            serial: None,
            input_text: String::new(),
            #[cfg(windows)]
            input_line_end: "\r\n".to_string(),
            #[cfg(unix)]
            input_line_end: "\n".to_string(),
            output_text: String::new(),
            auto_scroll: true,
            hex_output: false,
            tx_cnt: 0,
            rx_cnt: 0,
            macros: Macros::new(),
            macros_ui_open: true,
            logger_window_open: false,
            about_window_open: false,
            macros_window_open: false,
        };

        if app.port.is_empty() && !app.available_ports.is_empty() {
            app.port = app.available_ports[0].clone();
        }

        app
    }

    fn render_ui(&mut self, ctx: &egui::Context) {
        self.render_menu_bar(ctx);
        self.render_status_bar(ctx);
        self.render_main_area(ctx);
    }

    fn render_windows(&mut self, ctx: &egui::Context) {
        logger_window::render_window(&mut self.logger_window_open, ctx);
        about_window::render_window(&mut self.about_window_open, ctx);
        macros_window::render_window(&mut self.macros_window_open, ctx);
    }

    fn handle_serial_data(&mut self) {
        if let Some(serial) = &self.serial {
            if let Some(s) = serial.recv() {
                self.rx_cnt += s.len();

                if self.hex_output {
                    let mut hex_string = String::new();
                    for byte in s.as_bytes() {
                        if let Err(e) = write!(hex_string, "{:02X} ", byte) {
                            error!("Error writing hex string: {e:?}");
                        }
                    }
                    self.output_text.push_str(&hex_string);
                } else {
                    self.output_text.push_str(&s);
                }
            }
        }
    }

    pub fn send(&mut self) {
        if let Some(serial) = &self.serial {
            let s = format!("{}{}", self.input_text, self.input_line_end);
            let len = s.len();

            match serial.send(SerialMsg::Str(s)) {
                Ok(_) => self.tx_cnt += len,
                Err(e) => error!("Failed to send serial data: {e:?}")
            }
        }
    }

    pub fn connect(&mut self) {
        match SerialMainState::new(&self.port, self.serial_settings) {
            Ok(serial) => {
                self.serial = Some(serial);
                info!("Opened serial port {}", self.port);
            }
            Err(e) => error!("Couldn't open serial port {}: {e:?}", self.port)
        }
    }

    pub fn disconnect(&mut self) {
        if let Some(serial) = self.serial.take() {
            if let Err(e) = serial.close() {
                error!("Couldn't close serial port: {e:?}");
            }
        }
    }

    pub fn cut(&mut self) {
        if let Ok(mut clipboard) = ClipboardContext::new() {
            clipboard
                .set_contents(self.output_text.clone())
                .unwrap_or_default();
            self.output_text.clear();
        }
    }

    pub fn copy(&mut self) {
        if let Ok(mut clipboard) = ClipboardContext::new() {
            clipboard
                .set_contents(self.output_text.clone())
                .unwrap_or_default();
        }
    }

    pub fn paste(&mut self) {
        if let Ok(mut clipboard) = ClipboardContext::new() {
            self.input_text
                .push_str(&clipboard.get_contents().unwrap_or_default());
        }
    }

    pub fn quit(&mut self) {
        self.ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_serial_data();

        self.render_ui(ctx);
        self.render_windows(ctx);

        ctx.request_repaint_after(Duration::from_millis(50));
    }
}

pub fn run(port: String, settings: SerialSettings) -> anyhow::Result<()> {
    Logger::init()?;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800f32, 800f32]),
        ..Default::default()
    };

    eframe::run_native(
        "Rustcom",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(port, settings, cc)))),
    )
    .map_err(|_| anyhow::anyhow!(AppError::Other))
}
