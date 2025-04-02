use crate::logger::Logger;
use crate::macros::Macros;
use crate::serial::{SerialMainState, SerialMsg, SerialSettings};
use crate::util;
use anyhow::{Result, anyhow, bail};
use clipboard::{ClipboardContext, ClipboardProvider};
use crossbeam::channel::{Receiver, Sender, unbounded};
use directories::BaseDirs;
use log::error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

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

    pub macros: Macros,
    pub macro_channel: (Sender<String>, Receiver<String>),
    pub macros_ui_open: bool,
    pub logger_window_open: bool,
    pub about_window_open: bool,
    pub macros_window_open: bool,

    pub logging_to_file_started: bool,
    pub log_file_append: bool,
    pub log_file_name: String,
    pub logging_thread_stop_sig: Arc<AtomicBool>,
    pub logging_sender: Option<Sender<String>>,
}

impl App {
    pub fn new(serial_settings: SerialSettings, cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Light);

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
            macros: Macros::new(),
            macro_channel: unbounded(),
            macros_ui_open: true,
            logger_window_open: false,
            about_window_open: false,
            macros_window_open: false,
            logging_to_file_started: false,
            log_file_append: false,
            log_file_name,
            logging_thread_stop_sig: Arc::new(AtomicBool::new(false)),
            logging_sender: None,
        }
    }

    fn render_ui(&mut self, ctx: &egui::Context) {
        self.render_menu_bar(ctx);
        self.render_status_bar(ctx);
        self.render_main_area(ctx);
    }

    fn show_windows(&mut self, ctx: &egui::Context) {
        self.show_logger_window(ctx);
        self.show_about_window(ctx);
        self.show_macros_window(ctx);
    }

    fn handle_serial_data(&mut self) -> Result<()> {
        let Some(serial) = &self.serial else {
            return Ok(());
        };

        let Some(data) = serial.recv() else {
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

    fn handle_macros_data(&mut self) -> Result<()> {
        match self.macro_channel.1.try_recv() {
            Ok(data) => {
                self.serial_send(SerialMsg::Str(data))?;
            }
            Err(crossbeam::channel::TryRecvError::Empty) => {
                return Ok(());
            }
            Err(crossbeam::channel::TryRecvError::Disconnected) => {
                bail!("Macro channel disconnected")
            }
        }
        Ok(())
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

    pub fn cut(&mut self) {
        self.copy();

        self.output_text.clear();
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

    pub fn quit(&self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    pub fn start_macro(&mut self, num: usize) {
        let Some(m) = self.macros.macros.get(num) else {
            return;
        };

        let Some(s) = self.macros.stop_signals.get(num) else {
            return;
        };

        let stop_signal = s.clone();
        let macro_text = m.text.clone();
        let delay = m.delay;

        stop_signal.store(true, Ordering::SeqCst);

        let sender = self.macro_channel.0.clone();

        let handle = thread::spawn(move || {
            while stop_signal.load(Ordering::SeqCst) {
                sender.send(macro_text.clone() + "\n").ok();
                thread::sleep(Duration::from_millis(delay as u64));
            }
        });

        self.macros.running_threads[num] = Some(handle);
    }

    pub fn stop_macro(&mut self, num: usize) {
        let Some(stop_signal) = self.macros.stop_signals.get(num) else {
            return;
        };

        let Some(handle) = self.macros.running_threads.get_mut(num) else {
            return;
        };

        stop_signal.store(false, Ordering::SeqCst);
        handle.take();
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
                if let Ok(s) = receiver.recv() {
                    if let Err(e) = file.write_all(s.as_bytes()) {
                        error!("Can't write to log file: {e:?}");
                    }
                }
            }
        });
    }

    pub fn stop_recording_thread(&mut self) {
        self.logging_thread_stop_sig.store(false, Ordering::SeqCst);
        self.logging_sender.take();
        self.logging_to_file_started = false;
    }

    pub fn update(&mut self, ctx: &egui::Context) -> Result<()> {
        self.handle_serial_data()?;
        self.handle_macros_data()?;

        self.render_ui(ctx);
        self.show_windows(ctx);

        Ok(())
    }

    pub fn set_target_fps(ctx: &egui::Context, fps: u32) {
        ctx.request_repaint_after(util::calculate_repaint_duration(fps));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Err(e) = self.update(ctx) {
            error!("Error during frame update: {e:?}");
        }

        App::set_target_fps(ctx, 60);
    }
}

pub fn run(settings: SerialSettings) -> Result<()> {
    Logger::init()?;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800f32, 800f32]),
        ..Default::default()
    };

    eframe::run_native(
        "Rustcom",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(settings, cc)))),
    )
    .map_err(|e| anyhow!("Error during run_native: {e:?}"))?;

    Ok(())
}
