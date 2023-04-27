use flume::Sender;
use join_str::jstr;
use egui::{Color32};
use color_hex::color_from_hex;
use log::{Log, Record};
use crate::gui::Message;
use std::sync::Mutex;

use once_cell::sync::{OnceCell};

macro_rules! hex_color {
    ($hex:expr) => {{
        let _arr = color_from_hex!($hex);
        Color32::from_rgb(_arr[0], _arr[1], _arr[2])
    }};
}

pub const _GREEN: Color32 = hex_color!("#528f24");
pub const _BLUE: Color32 = hex_color!("#38b6f1");
pub const _RED: Color32 = hex_color!("#F52331");
pub const _YELLOW: Color32 = hex_color!("#ffbc28");
pub const _ORANGE: Color32 = hex_color!("#ff953f");


#[derive(Debug, Clone)]
pub struct Entry {
    pub timestamp: String,
    pub level: String,
    pub target: String,
    pub args: String,
}

impl Entry {
    pub fn format(&self, job: &mut String, _dark_mode: bool) {
        job.push_str(&jstr!("[{&self.timestamp}] "));
        job.push_str(&jstr!("{&self.level} "));
        job.push_str(&self.args);
        job.push('\n');
    }
}

impl From<&Record<'_>> for Entry {
    fn from(record: &Record) -> Self {
        Self {
            // timestamp: astrolabe::DateTime::now().format("y-MM-dd h:mm:ss"),
            timestamp: astrolabe::DateTime::now().format("hh:mm:ss"),
            level: record.level().to_string(),
            target: record.target().to_string(),
            args: format!("{:?}", record.args()),
        }
    }
}

pub static LOGGER: OnceCell<Logger> = OnceCell::new();

pub fn init() {
    log::set_logger(Logger::global()).unwrap();
    let level = LOGGER.get().unwrap().inner.filter();
    log::set_max_level(level.max(log::LevelFilter::Debug));
}

#[derive(Debug)]
pub struct Logger {
    inner: env_logger::Logger,
    sender: OnceCell<Sender<Message>>,
    queue: Mutex<Vec<Entry>>,
}

impl Logger {
    pub fn global() -> &'static Logger {
        LOGGER.get().expect("logger is not initialized")
    }

    pub fn new() -> Self {
        Self {
            inner: env_logger::builder().build(),
            sender: OnceCell::new(),
            queue: Mutex::new(Vec::new()),
        }
    }

    fn flush_queue(&self) {
        let mut queue = self.queue.lock().unwrap();

        if queue.len() > 1000 {
            queue.drain(..500).count();
        }

        for entry in queue.drain(..) {
            self.sender.get().unwrap().send(Message::Log(entry)).unwrap();
        }
    }

    pub fn set_sender(&self, sender: Sender<Message>) {
        self.sender.set(sender).unwrap();
        self.flush_queue();
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        let entry: Entry = record.into();

        if let Some(sender) = self.sender.get() {
            sender.send(Message::Log(entry)).unwrap();
        }

        if self.enabled(record.metadata()) {
            self.inner.log(record);
        }
    }

    fn flush(&self) {
        self.flush_queue();
        self.inner.flush();
    }
}