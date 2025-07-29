mod osu_parser;
mod osu_util;
mod file_tools;
mod constants;

use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use crate::osu_parser::OsuParser;

struct MyApp {
    selected_file: Option<PathBuf>,
    output_file: Option<String>,
    offset: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { selected_file: None, output_file: None, offset: 0.0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Osu to SSC Converter");

            if ui.button("Select .osu File").clicked() {
                if let Some(path) = FileDialog::new().add_filter("Osu Beatmap", &["osu"]).pick_file() {
                    self.selected_file = Some(path);
                }
            }

            if let Some(ref path) = self.selected_file {
                ui.label(format!("Selected File: {:?}", path.display()));

                ui.horizontal(|ui| {
                    ui.label("Enter Offset:");
                    ui.add(egui::DragValue::new(&mut self.offset).speed(0.1));
                });

                if ui.button("Convert to SSC").clicked() {
                    let file_path = path.to_string_lossy().to_string();
                    let mut parser = OsuParser::new(file_path.clone());
                    let file_data = parser.parse_file();
                    let output_path = format!("{}.ssc", file_path.trim_end_matches(".osu"));

                    parser.write_chart(&file_data, &output_path, self.offset);
                    self.output_file = Some(output_path);
                }
            }

            if let Some(ref output) = self.output_file {
                ui.label(format!("Output File: {}", output));
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Osu to SSC Converter", options, Box::new(|_cc| Ok(Box::new(MyApp::default())))).unwrap();
}
