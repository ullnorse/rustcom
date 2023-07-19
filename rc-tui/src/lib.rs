use anyhow::Result;
use rc_core::serial::SerialConfig;

pub fn run(_device: String, _config: SerialConfig) -> Result<()> {
    println!("Hello from tui");

    Ok(())
}
