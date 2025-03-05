use anyhow::Result;
use rustcom::app;
use rustcom::cli;

fn main() -> Result<()> {
    let (device, settings) = cli::run()?;
    app::run(device, settings)
}
