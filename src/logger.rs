use anyhow::Result;
use log::Level;
use std::convert::TryFrom;
use std::sync::{
    LazyLock, Mutex,
    atomic::{AtomicUsize, Ordering},
};

pub static LOGGER: LazyLock<Logger> = LazyLock::new(Logger::new);

pub fn init() -> Result<()> {
    log::set_logger(&*LOGGER)?;
    log::set_max_level(log::LevelFilter::Trace);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevelUsize {
    Trace = 5,
    Debug = 4,
    Info = 3,
    Warn = 2,
    Error_ = 1,
}

impl From<Level> for LogLevelUsize {
    fn from(level: Level) -> Self {
        match level {
            Level::Trace => LogLevelUsize::Trace,
            Level::Debug => LogLevelUsize::Debug,
            Level::Info => LogLevelUsize::Info,
            Level::Warn => LogLevelUsize::Warn,
            Level::Error => LogLevelUsize::Error_,
        }
    }
}

impl From<LogLevelUsize> for Level {
    fn from(level: LogLevelUsize) -> Self {
        match level {
            LogLevelUsize::Trace => Level::Trace,
            LogLevelUsize::Debug => Level::Debug,
            LogLevelUsize::Info => Level::Info,
            LogLevelUsize::Warn => Level::Warn,
            LogLevelUsize::Error_ => Level::Error,
        }
    }
}

impl TryFrom<usize> for LogLevelUsize {
    type Error = ();

    fn try_from(value: usize) -> std::result::Result<Self, Self::Error> {
        match value {
            5 => Ok(LogLevelUsize::Trace),
            4 => Ok(LogLevelUsize::Debug),
            3 => Ok(LogLevelUsize::Info),
            2 => Ok(LogLevelUsize::Warn),
            1 => Ok(LogLevelUsize::Error_),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Logger {
    buffer: Mutex<String>,
    level: AtomicUsize,
}

impl Logger {
    fn new() -> Self {
        Self {
            buffer: Mutex::new(String::new()),
            level: AtomicUsize::new(LogLevelUsize::Info as usize),
        }
    }

    pub fn set_level(&self, level: Level) {
        self.level
            .store(LogLevelUsize::from(level) as usize, Ordering::Relaxed);
    }

    pub fn get_level(&self) -> Option<Level> {
        let level_usize = self.level.load(Ordering::Relaxed);
        LogLevelUsize::try_from(level_usize).ok().map(Level::from)
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
        let current_level = self.level.load(Ordering::Relaxed);
        let metadata_level = LogLevelUsize::from(metadata.level()) as usize;
        metadata_level <= current_level
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
