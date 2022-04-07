use eframe::{epi::App, run_native, NativeOptions};
use egui::containers::panel::CentralPanel;
use egui::containers::ScrollArea;
use egui::{Button, Color32, RichText};
use std::fs;
struct Editor {
    files: Vec<String>,
}

impl Editor {
    fn new() -> Self {
        Self {
            files: fs::read_dir(".")
                .unwrap()
                .map(|x| x.unwrap().file_name().to_string_lossy().to_string())
                .collect(),
        }
    }
}

impl App for Editor {
    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |uid| {
                for file in &self.files {
                    uid.add(
                        Button::new(RichText::new(file.clone()).color(Color32::WHITE))
                            .fill(Color32::BLACK),
                    );
                }
            });
        });
    }
    fn name(&self) -> &str {
        "Code Editor"
    }
}

fn main() {
    let app = Editor::new();
    let native_options = NativeOptions::default();
    run_native(Box::new(app), native_options);
}
