mod cli;
mod serial;
mod tui;
mod gui;
mod file_sender;
mod logger;

fn main() {
    cli::Cli::run();
}
