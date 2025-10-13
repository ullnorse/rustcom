pub mod app;
pub mod cli;
pub mod clipboard;
pub mod logger;
pub mod serial;
pub mod ui;
pub mod util;
pub mod error;

use crate::app::App;
use eframe::egui::ViewportBuilder;
use crate::error::Result;

pub fn run() -> Result<()> {
    logger::init()?;

    let settings = cli::run();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([800f32, 800f32]),
        ..Default::default()
    };

    eframe::run_native(
        "Rustcom",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(settings, Some(cc))))),
    )?;

    Ok(())
}
