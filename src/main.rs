use arboard::{Clipboard, Error as ClipboardError, ImageData};
use eframe::{egui, NativeOptions};
use egui::Ui;

#[derive(Clone)]
enum ClipBoardContent { 
  TextContent(String), 
  ImageContent(ImageData<'static>),
}

#[derive(Clone)]
struct Clip {
  id: usize,
  content: Option<ClipBoardContent>,
  is_code: bool,
  pinned: bool,
}

impl Default for Clip {
  fn default() -> Self {
    Self {
      id: 0,
      content: None,
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
    Box::new(|_cc| Box::new(Sloth::default())),
  );
}

struct Sloth {
  clips: Vec<Clip>,
  saved_clips: Vec<Clip>,
}

impl Default for Sloth {
  fn default() -> Self {
    Self { 
      clips: Vec::new(),
      saved_clips: Vec::new(),
    }
  }
}

impl eframe::App for Sloth {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show_viewport(ui, |ui, _viewport| {
          ui.vertical(|ui: &mut Ui| {
            self.clips.iter_mut().for_each(|clip: &mut Clip| {
              if let Some(content) = &clip.content {
                match  content {
                  ClipBoardContent::TextContent(text) => {
                    ui.label(text.clone());
                    egui::Grid::new(format!("row_{}",clip.id))
                      .num_columns(3)
                      .min_col_width(100.0)
                      .show(ui, |ui| {
                        if ui.button("Copy").clicked() {
                          let mut clipboard: Clipboard = Clipboard::new().unwrap();
                          clipboard.set_text(text.clone()).unwrap();
                        };

                        if ui.button("Code").clicked() {
                          clip.is_code = true;
                        };

                        if !clip.pinned {
                          if ui.button("  Pin  ").clicked() {
                            let new_clip = Clip {
                              id: self.saved_clips.len(),
                              content: Some(ClipBoardContent::TextContent(text.clone())),
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
                            self.saved_clips.retain(|c| c.id != clip.id);
                          };
                        }
                        ui.end_row();
                    });
                    ui.add(egui::Separator::default());
                  },
                  ClipBoardContent::ImageContent(_image) => {
                  },
                } 
              }
            });
          });
        });
    });

    let last_clip_content = self.clips.last()
      .map_or_else(|| Some(ClipBoardContent::TextContent("".to_string())), |clip| clip.content.clone());

    if let Some(clip) = get_clip_to_add(&last_clip_content) {
      let new_clip = Clip {
        id: self.clips.len(),
        content: Some(clip),
        is_code: false,
        pinned: false,
      };
      self.clips.push(new_clip);
    }
  }
}


fn get_clipboard_content() -> Option<ClipBoardContent> {
  let mut clipboard: Clipboard = Clipboard::new().unwrap();
  let text: Result<String, ClipboardError> = clipboard.get_text();
  let image: Result<ImageData<'static>, ClipboardError> = clipboard.get_image();

  if let Ok(text) = text {
    return Some(ClipBoardContent::TextContent(text))
  } else if let Ok(image) = image {
    return Some(ClipBoardContent::ImageContent(image))
  }

  None
}

fn get_clip_to_add(last_clip_content: &Option<ClipBoardContent>) -> Option<ClipBoardContent> {
  let clipboard_content = get_clipboard_content();
  if let Some(content) = clipboard_content {
    match content {
      ClipBoardContent::TextContent(text) => {
        if let Some(last_content) = last_clip_content {
          match last_content {
            ClipBoardContent::TextContent(last_text) => {
              if last_text != &text {
                return Some(ClipBoardContent::TextContent(text));
              }
            },
            _ => return None,
          }
        }
      },
      ClipBoardContent::ImageContent(_image) => return None,
    }
  }

  None
}