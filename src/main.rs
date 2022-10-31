#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[macro_use]
extern crate lazy_static;

pub mod finlibs;
use std::fs;
use finlibs::finance;
use rfd::FileDialog;
use finlibs::envs;


use eframe::egui;

use crate::finlibs::utils;


fn main() {
    let mut options = eframe::NativeOptions::default();
    options.transparent = true;
    options.initial_window_size.replace(egui::Vec2::new(400.0, 600.0));
  
    eframe::run_native(
        "FinTools",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    isins: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            isins: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.heading("ISIN Converter");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui.button("Compare").clicked() {
                        self.compare();
                    }
                });                
            });            
            ui.add_space(10.0);
            ui.vertical(|ui| {
                ui.set_max_height(500.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Add a lot of widgets here.
                    let maxsize = ui.available_size();
                    ui.add_sized(maxsize, egui::TextEdit::multiline(&mut self.isins));
                });
            });
            ui.add_space(10.0);
            if ui.button("Convert").clicked() {
                self.convert();
            }
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}

impl MyApp {
    fn convert(&self) -> Option<i32> {
        if self.isins.len() < 12 { 
            Some(0)
        } else {
            let lines = self.isins.lines();

            let line_vec: Vec<&str> = lines.into_iter().collect();
            let map = finance::request_symbols(line_vec);

            if map.len() > 0 {
                let out_str = map.iter()
                    .map(|n| n.1)
                    .fold(String::new(), | acc, x| acc + x + "\n");


                let filename = envs::get_config().prefix_stocklist + &utils::formatted_timestamp() + ".txt";
            
                let file_path = FileDialog::new()
                    .add_filter("text", &["txt"])
                    .set_directory(envs::get_config().output_path)
                    .set_file_name(&filename)
                    .pick_folder();

                if file_path.is_some() {
                    let mut fp = file_path.unwrap();
                    fp.push(&filename);

                    fs::write(fp, out_str)
                        .expect("Should have been able to write the file");
                }
            }
            Some(map.len() as i32)
        }
    }

    fn compare(&self) {

    }
}