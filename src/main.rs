pub mod osu;
mod parser;
mod utils;

use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use crate::parser::osu_parser::OsuParserV2;

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
                    ui.label("Enter Offset (-1 * osu_offset/1000):");
                    ui.add(egui::DragValue::new(&mut self.offset).speed(0.1));
                });

                ui.label("Convert to SSC");
                if ui.button("Parse to SSC").clicked() {
                    let file_path = path.to_string_lossy().to_string();
                    let parser_v2 = OsuParserV2::new(file_path.clone());
                    let output_path = format!("{}.ssc", file_path.trim_end_matches(".osu"));
                    parser_v2.write_chart(&output_path);
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
