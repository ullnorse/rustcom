mod menu_bar;
mod status_bar;
mod tabs;

use anyhow::Result;
use base::serial::{SerialConfig, Serial};
use eframe::egui::Visuals;
use egui_dock::{DockArea, DockState, Style};

use crate::tabs::{Tab, default_ui};

use std::sync::RwLock;

use std::rc::Rc;

struct App {
    tree: Rc<RwLock<DockState<Box<dyn Tab>>>>,

    current_serial_device: String,
    serial_devices: Vec<String>,
    serial_config: SerialConfig,
    device_connected: bool,

    tx_cnt: u32,
    rx_cnt: u32,
}

impl App {
    fn new(_cc: &eframe::CreationContext, device: String, serial_config: SerialConfig) -> Self {
        let mut app = Self {
            tree: Rc::new(RwLock::new(default_ui())),

            current_serial_device: String::new(),
            serial_devices: Serial::available_ports().unwrap(),
            serial_config: serial_config.clone(),
            device_connected: false,

            tx_cnt: 0,
            rx_cnt: 0,
        };

        app.current_serial_device = if !device.is_empty() {
            device
        } else if !app.serial_devices.is_empty() {
            app.serial_devices[0].clone()
        } else {
            String::new()
        };

        app
    }

    fn render_main_area(&mut self, ctx: &eframe::egui::Context) {
        DockArea::new(self.tree.clone().write().as_deref_mut().unwrap())
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, self)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.render_menu_bar(ctx);
        self.render_status_bar(ctx);
        self.render_main_area(ctx);
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