#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chrono::Local;
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
        Box::new(move |ctx| Box::new(TimeLeft::new(&ctx))),
    )
}

struct TimeLeft {
    time: Arc<Mutex<chrono::DateTime<Local>>>,
}

impl TimeLeft {
    fn new(_ctx: &eframe::CreationContext) -> Self {
        let time = Arc::new(Mutex::new(Local::now()));
        let time_clone = Arc::clone(&time);
        thread::spawn(move || loop {
            let mut t = time_clone.lock().unwrap();
            *t = Local::now();
            drop(t);
            log::debug!(
                "{}",
                Arc::new(Local::now())
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            );
            thread::sleep(Duration::from_secs(1));
        });

        Self { time }
    }
}

impl eframe::App for TimeLeft {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let t = self.time.lock().unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("{}", t.format("%Y-%m-%d %H:%M:%S").to_string()));
        });
    }
}
