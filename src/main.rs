use arboard::{Clipboard, Error as ClipboardError};
use eframe::{egui, NativeOptions};
use egui::Ui;

#[derive(PartialEq, Clone)]
enum Kind {
  TextContent,
  _ImageContent,
  CodeContent,
}

struct Clip {
  kind: Kind,
  content: String,
  is_code: bool,
  pinned: bool,
}

impl Default for Clip {
  fn default() -> Self {
    Self {
      kind: Kind::TextContent,
      content: "".to_string(),
      is_code: false,
      pinned: false,
    }
  }
}

fn main() {
  let options: NativeOptions = NativeOptions {
    initial_window_size: Some(egui::Vec2::new(300.0, 600.0)),
    ..eframe::NativeOptions::default()
  };

  eframe::run_native(
    "Sloth 🦥",
    options,
    Box::new(|_cc| Box::new(MyApp::default())),
  );
}

struct MyApp {
  clips: Vec<Clip>,
  saved_clips: Vec<Clip>,
}

impl Default for MyApp {
  fn default() -> Self {
    Self { 
      clips: Vec::new(),
      saved_clips: Vec::new(),
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show_viewport(ui, |ui, _viewport| {
          ui.vertical(|ui: &mut Ui| {
            self.clips.iter_mut().for_each(|clip: &mut Clip| {
              if clip.kind == Kind::TextContent {
                ui.label(clip.content.clone());
              } else if clip.kind == Kind::CodeContent {
                ui.code(clip.content.clone());
              } 
    
              ui.horizontal(|ui| {
                if ui.button("Copy").clicked() {
                  let mut clipboard: Clipboard = Clipboard::new().unwrap();
                  clipboard.set_text(clip.content.to_string()).unwrap();
                };
                if ui.button("Code").clicked() {
                  clip.is_code = true;
                };
                if !clip.pinned {
                  if ui.button("  Pin  ").clicked() {
                    let new_clip = Clip {
                      kind: clip.kind.clone(),
                      content: clip.content.clone(),
                      is_code: clip.is_code,
                      pinned: true,
                    };
                    clip.pinned = true;
                    self.saved_clips.push(new_clip);
                  };  
                }
                if clip.pinned {
                  if ui.button("Unpin").clicked() {
                    clip.pinned = false;
                    self.saved_clips.retain(|c| c.content != clip.content);
                  };
                }
              });
    
              ui.add(egui::Separator::default());
            });
          });
        });

      let mut clipboard: Clipboard = Clipboard::new().unwrap();
      let content_clipboard: Result<String, ClipboardError> = clipboard.get_text();
      if let Ok(content) = content_clipboard {
        let last: Option<&Clip> = self.clips.last();
        if last.is_none() || &last.unwrap().content != &content {
          let mut clip: Clip = Clip::default();
          clip.content = content;
          self.clips.push(clip);
        }
      }
      });
  }
}
