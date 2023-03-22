mod cli;
mod serial;
mod tui;
mod gui;
mod tabs;

fn main() {
    if let Err(e) = std::panic::catch_unwind(cli::Cli::run) {
        println!(
            "An unrecoverable error occured. Error details: {}",
            e.downcast::<String>()
                .or_else(|e| e.downcast::<&'static str>().map(|s| Box::new((*s).into())))
                .unwrap_or_else(|_| {
                    Box::new(
                        "An unknown error occured, check the log for possible details."
                            .to_string(),
                    )
                })
        );
    }
}
