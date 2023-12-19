#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chrono::{DateTime, Local, TimeZone};
use eframe::egui;
use egui::FontDefinitions;
use std::time;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let font_name = "NotoSansSC-Regular";
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        font_name.to_owned(),
        egui::FontData::from_static(include_bytes!("../NotoSansSC-Regular.otf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, font_name.to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push(font_name.to_owned());

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        always_on_top: true,
        transparent: true,
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "TimeLeft",
        options,
        Box::new(move |ctx| {
            ctx.egui_ctx.set_fonts(fonts);
            Box::new(TimeLeft {})
        }),
    )
}

struct TimeLeft {}

impl TimeLeft {}

impl eframe::App for TimeLeft {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let item = Item {
            time: chrono::DateTime::parse_from_rfc3339("2023-12-30T16:39:57+08:00")
                .unwrap()
                .with_timezone(&Local),
            item_type: ItemType::Default,
            describe: "test".to_string(),
        };

        let item2 = Item {
            time: chrono::DateTime::parse_from_rfc3339("2023-11-30T16:39:57+08:00")
                .unwrap()
                .with_timezone(&Local),
            item_type: ItemType::Default,
            describe: "test".to_string(),
        };

        let item3 = Item {
            time: chrono::DateTime::parse_from_rfc3339("2023-11-30T19:00:00+08:00")
                .unwrap()
                .with_timezone(&Local),
            item_type: ItemType::EveryDay,
            describe: "test".to_string(),
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("{}", item.calculate_duration()));
            ui.label(format!("{}", item2.calculate_duration()));
            ui.label(format!("{}", item3.calculate_duration()));
        });
        //ctx.request_repaint();
        ctx.request_repaint_after(time::Duration::from_secs(1));
    }
}

struct Item {
    pub time: DateTime<Local>,
    pub item_type: ItemType,
    pub describe: String,
}

enum ItemType {
    EveryDay,
    Default,
}

impl Item {
    pub fn calculate_duration(&self) -> String {
        let t = Local::now();
        let to_time = match self.item_type {
            ItemType::EveryDay => {
                let tt = chrono::NaiveDateTime::new(t.date_naive(), self.time.time());
                Local.from_local_datetime(&tt).earliest().unwrap()
            }
            _ => self.time.clone(),
        };
        let r = t.signed_duration_since(to_time);
        let mark = {
            if r < chrono::Duration::zero() {
                -1
            } else {
                1
            }
        };
        let r = r.abs();
        if mark == -1 {
            format!(
                "距离{}还有{}天{}小时{}分{}秒",
                self.describe,
                r.num_days(),
                r.num_hours() % 24,
                r.num_minutes() % 60,
                r.num_seconds() % 60
            )
        } else {
            format!(
                "距离{}已经过去了{}天{}小时{}分{}秒",
                self.describe,
                r.num_days(),
                r.num_hours() % 24,
                r.num_minutes() % 60,
                r.num_seconds() % 60
            )
        }
    }
}
