use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("eframe error")]
    Eframe(#[from] eframe::Error),
    #[error("couldn't initialize logger")]
    Logger(#[from] log::SetLoggerError),
}

pub type Result<T> = std::result::Result<T, AppError>;