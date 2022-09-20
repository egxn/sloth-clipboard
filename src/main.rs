use arboard::{Clipboard, Error as ClipboardError};
use eframe::{egui, NativeOptions};

fn main() {   
    let options: NativeOptions = eframe::NativeOptions::default();

    eframe::run_native(
        "Sloth 🦥",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}


struct MyApp {
    clips: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            clips: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
  

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(300.0);
            ui.set_height(ui.available_height());
            ui.vertical(|ui| {
                self.clips.iter().for_each(|clip| {
                    ui.label(clip);
                });
            });

            let mut clipboard: Clipboard = Clipboard::new().unwrap();
            let text:Result<String, ClipboardError> = clipboard.get_text();
            if let Ok(text ) = text {
                let last: Option<&String> = self.clips.last();
                if last.is_none() || last.unwrap() != &text {
                    self.clips.push(text);
                }
            }
        });
    }
}
