use crate::logger::Logger;
use crate::macros::Macros;
use crate::serial::{SerialMainState, SerialMsg, SerialSettings};
use anyhow::{Result, anyhow, bail};
use clipboard::{ClipboardContext, ClipboardProvider};
use crossbeam::channel::{Receiver, Sender, unbounded};
use log::{error, info};
use std::fmt::Write;
use std::sync::atomic::Ordering;
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

        Self {
            serial_settings: settings,
            available_ports,
            serial: None,
            input_text: String::new(),
            input_line_end: Self::default_line_end(),
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

    fn handle_serial_data(&mut self) {
        let Some(serial) = &self.serial else {
            return;
        };

        let Some(data) = serial.recv() else {
            return;
        };

        self.rx_cnt += data.len();

        let output_text = if self.hex_output {
            let mut hex_string = String::new();
            for byte in data.as_bytes() {
                if let Err(e) = write!(hex_string, "{:02X} ", byte) {
                    error!("Error writing hex string: {e:?}");
                }
            }
            hex_string
        } else {
            data
        };

        self.output_text.push_str(&output_text);
    }

    fn handle_macros_data(&mut self) {
        let Ok(data) = self.macro_channel.1.try_recv() else {
            return;
        };

        if let Err(e) = self.serial_send(SerialMsg::Str(data)) {
            error!("Failed to send macro data: {e:?}");
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

    pub fn connect(&mut self) {
        match SerialMainState::new(self.serial_settings.clone()) {
            Ok(serial) => {
                self.serial = Some(serial);
                info!("Opened serial port {}", self.serial_settings.port);
            }
            Err(e) => error!(
                "Couldn't open serial port {}: {e:?}",
                self.serial_settings.port
            ),
        }
    }

    pub fn disconnect(&mut self) {
        self.serial.take();
        info!("Closed serial port: {}", self.serial_settings.port);
    }

    pub fn cut(&mut self) {
        self.copy();

        self.output_text.clear();
    }

    pub fn copy(&mut self) {
        let Ok(mut clipboard) = ClipboardContext::new() else {
            return;
        };

        clipboard
            .set_contents(self.output_text.clone())
            .unwrap_or_default();
    }

    pub fn paste(&mut self) {
        let Ok(mut clipboard) = ClipboardContext::new() else {
            return;
        };

        self.input_text
            .push_str(&clipboard.get_contents().unwrap_or_default());
    }

    pub fn quit(&self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn default_line_end() -> String {
        #[cfg(not(unix))]
        {
            "\r\n".to_string()
        }

        #[cfg(unix)]
        {
            "\n".to_string()
        }
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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_serial_data();
        self.handle_macros_data();

        self.render_ui(ctx);
        self.show_windows(ctx);

        ctx.request_repaint_after(Duration::from_millis(50));
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
