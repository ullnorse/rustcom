use anyhow::Result;
use rustcom::app;
use rustcom::cli;

fn main() -> Result<()> {
    app::run(cli::run()?)
}
