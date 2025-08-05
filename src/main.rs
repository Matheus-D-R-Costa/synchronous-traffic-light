mod state;
mod ui;
mod app;
mod constants;

use crate::app::TrafficLightApp;
use eframe::{egui, NativeOptions};

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([500.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Synchronous Traffic Light GUI",
        options,
        Box::new(|_cc| Box::<TrafficLightApp>::default()),
    )
}