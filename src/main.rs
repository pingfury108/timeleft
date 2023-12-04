#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chrono::Local;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        always_on_top: true,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(
        "TimeLeft",
        options,
        Box::new(move |_ctx| Box::new(TimeLeft {})),
    )
}

struct TimeLeft {}

impl TimeLeft {}

impl eframe::App for TimeLeft {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let t = Local::now();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("{}", t.format("%Y-%m-%d %H:%M:%S")));
        });
        ctx.request_repaint();
    }
}
