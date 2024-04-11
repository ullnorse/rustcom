use base::{cli, cli::AppType};
use anyhow::Result;

fn main() -> Result<()> {
    match cli::run()? {
        (AppType::Tui, device, config) => tui::run(device, config)?,
        (AppType::Gui, device, config) => gui::run(device, config)?,
    }

    Ok(())
}