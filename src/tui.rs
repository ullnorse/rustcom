use crate::serial::serial_config::SerialConfig;
use anyhow::Result;

pub fn run(_device: &str, _config: SerialConfig) -> Result<()> {
    println!("Started TUI app");
    Ok(())
}
