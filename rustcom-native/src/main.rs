use rc_core::cli;
use anyhow::Result;

fn main() -> Result<()> {
    match cli::run()? {
        (cli::AppType::Tui, device, config) => rc_tui::run(device, config)?,
        (cli::AppType::Gui, device, config) => rc_gui::run(device, config)?,
    }

    Ok(())
}
