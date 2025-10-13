use crate::serial::{SerialMainState, SerialMsg, SerialSettings};
use crate::util;
use anyhow::{Result, anyhow, bail};
use clipboard::{ClipboardContext, ClipboardProvider};
use crossbeam::channel::{Sender, unbounded};
use directories::BaseDirs;
use eframe::egui::{Context, Theme, ViewportCommand};
use log::error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct App {
    pub serial_settings: SerialSettings,
    pub available_ports: Vec<String>,
    pub serial: Option<SerialMainState>,

    pub input_text: String,
    pub input_line_end: String,
    pub output_text: String,

    pub auto_scroll: bool,
    pub hex_output: bool,

    pub tx_cnt: usize,
    pub rx_cnt: usize,

    pub logger_window_open: bool,
    pub about_window_open: bool,

    pub logging_to_file_started: bool,
    pub log_file_append: bool,
    pub log_file_name: String,
    pub logging_thread_stop_sig: Arc<AtomicBool>,
    pub logging_sender: Option<Sender<String>>,
    pub clipboard: ClipboardContext,
}

impl App {
    pub fn new(serial_settings: SerialSettings, cc: Option<&eframe::CreationContext>) -> Self {
        let egui_ctx = Context::default();
        let egui_ctx = cc.map(|cc| &cc.egui_ctx).unwrap_or(&egui_ctx);
        egui_ctx.set_theme(Theme::Light);

        let available_ports = SerialMainState::available_ports();
        let selected_port = serial_settings
            .port
            .is_empty()
            .then(|| available_ports.first().cloned())
            .flatten()
            .unwrap_or_else(|| serial_settings.port.clone());

        let settings = SerialSettings::new(
            selected_port,
            serial_settings.baud_rate,
            serial_settings.data_bits,
            serial_settings.stop_bits,
            serial_settings.parity,
            serial_settings.flow_control,
        );

        let log_file_name = BaseDirs::new()
            .and_then(|dirs| {
                let mut path = dirs.home_dir().to_path_buf();
                path.push("rustcom.log");
                path.to_str().map(String::from)
            })
            .unwrap_or_default();

        Self {
            serial_settings: settings,
            available_ports,
            serial: None,
            input_text: String::new(),
            input_line_end: util::default_line_end().to_string(),
            output_text: String::new(),
            auto_scroll: true,
            hex_output: false,
            tx_cnt: 0,
            rx_cnt: 0,
            logger_window_open: false,
            about_window_open: false,
            logging_to_file_started: false,
            log_file_append: false,
            log_file_name,
            logging_thread_stop_sig: Arc::new(AtomicBool::new(false)),
            logging_sender: None,
            clipboard: ClipboardContext::new().unwrap(),
        }
    }

    fn render_ui(&mut self, ctx: &Context) {
        self.render_menu_bar(ctx);
        self.render_status_bar(ctx);
        self.render_main_area(ctx);
    }

    fn show_windows(&mut self, ctx: &Context) {
        self.show_logger_window(ctx);
        self.show_about_window(ctx);
    }

    fn handle_serial_data(&mut self) -> Result<()> {
        let Some(serial) = &self.serial else {
            return Ok(());
        };

        let Ok(data) = serial.recv() else {
            return Ok(());
        };

        self.rx_cnt += data.len();

        let output = self.prepare_output(data)?;

        self.send_data_to_output_text(&output);

        if self.logging_to_file_started {
            self.send_data_to_logging_thread(output)?
        }

        Ok(())
    }

    fn send_data_to_output_text(&mut self, output: &str) {
        self.output_text.push_str(output);
    }

    fn send_data_to_logging_thread(&mut self, data: String) -> Result<()> {
        let sender = self
            .logging_sender
            .as_ref()
            .ok_or_else(|| anyhow!("Logging sender not initialized"))?;

        sender
            .send(data)
            .map_err(|e| anyhow!("Failed to send data to logging thread: {}", e))?;

        Ok(())
    }

    fn prepare_output(&self, data: String) -> Result<String> {
        if self.hex_output {
            let mut hex_string = String::new();
            util::format_hex(data.as_bytes(), &mut hex_string)?;
            Ok(hex_string)
        } else {
            Ok(data)
        }
    }

    pub fn send(&mut self) {
        let s = format!("{}{}", self.input_text, self.input_line_end);
        let len = s.len();

        if let Err(e) = self.serial_send(SerialMsg::Str(s)) {
            error!("Failed to send serial data: {e:?}")
        } else {
            self.tx_cnt += len
        }
    }

    pub fn serial_send(&self, msg: SerialMsg) -> Result<()> {
        let Some(serial) = &self.serial else {
            bail!("Serial is not available");
        };

        serial.send(msg)
    }

    pub fn connect(&mut self) -> Result<()> {
        SerialMainState::new(self.serial_settings.clone())
            .map(|serial| self.serial = Some(serial))
            .map_err(|e| {
                anyhow!(
                    "Couldn't open serial port {}: {e:?}",
                    self.serial_settings.port
                )
            })
    }

    pub fn disconnect(&mut self) -> Result<()> {
        self.serial.take();
        self.stop_recording_thread();
        Ok(())
    }

    pub fn quit(&self, ctx: &Context) {
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }

    pub fn start_recording_thread(&mut self) {
        let log_file_name = self.log_file_name.clone();
        let stop_sig = self.logging_thread_stop_sig.clone();
        stop_sig.store(true, Ordering::SeqCst);
        let log_file_append = self.log_file_append;
        let (sender, receiver) = unbounded();

        self.logging_sender = Some(sender);

        thread::spawn(move || {
            let file_result = if log_file_append {
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&log_file_name)
            } else {
                File::create(&log_file_name)
            };

            let mut file = match file_result {
                Ok(file) => file,
                Err(e) => {
                    error!("Error opening file for logging: {e:?}");
                    return;
                }
            };

            while stop_sig.load(Ordering::SeqCst) {
                if let Ok(s) = receiver.recv()
                    && let Err(e) = file.write_all(s.as_bytes())
                {
                    error!("Can't write to log file: {e:?}");
                }
            }
        });
    }

    pub fn stop_recording_thread(&mut self) {
        self.logging_thread_stop_sig.store(false, Ordering::SeqCst);
        self.logging_sender.take();
        self.logging_to_file_started = false;
    }

    pub fn update(&mut self, ctx: &Context) -> Result<()> {
        self.handle_serial_data()?;

        self.render_ui(ctx);
        self.show_windows(ctx);

        Ok(())
    }

    pub fn request_repaint_at_fps(ctx: &Context, fps: u32) {
        ctx.request_repaint_after(util::calculate_repaint_duration(fps));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if let Err(e) = self.update(ctx) {
            error!("Error during frame update: {e:?}");
        }

        App::request_repaint_at_fps(ctx, 60);
    }
}
