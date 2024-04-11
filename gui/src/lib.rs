mod menu_bar;
mod tabs;

use anyhow::Result;
use base::serial::SerialConfig;
use eframe::egui::{Visuals};
use egui_dock::{DockArea, DockState, Style};

use crate::tabs::{Tab, default_ui, TabViewer};

struct App {
    tree: DockState<Box<dyn Tab>>,
    tab_viewer: TabViewer
}

impl App {
    fn new(_cc: &eframe::CreationContext, _device: String, _config: SerialConfig) -> Self {
        Self {
            tree: default_ui(),
            tab_viewer: TabViewer
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.render_menu_bar(ctx);

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.tab_viewer)
    }
}

pub fn run(device: String, config: SerialConfig) -> Result<()> {

    let native_options = eframe::NativeOptions::default();

    let style = egui_dock::egui::Style {
        visuals: Visuals::light(),
        ..Default::default()
    };

    eframe::run_native("rustcom", native_options, Box::new(|cc| {
            cc.egui_ctx.set_style(style);
            Box::new(App::new(cc, device, config))
        })
    ).ok();

    Ok(())
}