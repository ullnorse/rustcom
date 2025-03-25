use anyhow::Result;
use log::Level;
use std::sync::{LazyLock, Mutex};

pub static LOGGER: LazyLock<Logger> = LazyLock::new(Logger::new);

#[derive(Debug)]
pub struct Logger {
    buffer: Mutex<String>,
    level: Mutex<Level>,
}

impl Logger {
    pub fn init() -> Result<()> {
        log::set_logger(&*LOGGER)?;
        log::set_max_level(log::LevelFilter::Trace);
        Ok(())
    }

    fn new() -> Self {
        Self {
            buffer: Mutex::new(String::new()),
            level: Mutex::new(Level::Info),
        }
    }

    pub fn set_level(&self, level: Level) {
        if let Ok(mut log_level) = self.level.lock() {
            *log_level = level;
        }
    }

    pub fn get_level(&self) -> Option<Level> {
        self.level.lock().ok().as_deref().cloned()
    }

    pub fn get_logs(&self) -> Option<String> {
        self.buffer.lock().ok().as_deref().cloned()
    }

    pub fn clear_logs(&self) {
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.clear();
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if let Ok(level) = self.level.lock() {
            return metadata.level() <= *level;
        }

        false
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let log_entry = format!("{} - {}\n", record.level(), record.args());

            if let Ok(mut buffer) = self.buffer.lock() {
                buffer.push_str(&log_entry);
            }
        }
    }

    fn flush(&self) {}
}
