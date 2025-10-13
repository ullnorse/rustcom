pub mod app;
pub mod cli;
pub mod clipboard;
pub mod logger;
pub mod serial;
pub mod ui;
pub mod util;

use crate::{app::App, logger::Logger};
use anyhow::{Result, anyhow};
use eframe::egui::ViewportBuilder;

pub fn run() -> Result<()> {
    Logger::init()?;

    let settings = cli::run()?;

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([800f32, 800f32]),
        ..Default::default()
    };

    eframe::run_native(
        "Rustcom",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(settings, Some(cc))))),
    )
    .map_err(|e| anyhow!("Error during run_native: {e:?}"))?;

    Ok(())
}
