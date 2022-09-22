use arboard::{Clipboard, Error as ClipboardError, ImageData};
use eframe::{egui, NativeOptions};
use egui::{Ui, ColorImage};
use image::{DynamicImage, ImageError, FlatSamples};

#[derive(Clone)]
enum ClipBoardContent { 
  TextContent(String), 
  ImageContent(ImageData<'static>),
}

#[derive(Clone)]
struct Clip {
  _id: usize,
  content: Option<ClipBoardContent>,
  _texture: Option<egui::TextureHandle>,
}

impl Default for Clip {
  fn default() -> Self {
    Self {
      _id: 0,
      content: None,
      _texture: None,
    }
  }
}

struct Sloth {
  clips: Vec<Clip>,
}

impl Default for Sloth {
  fn default() -> Self {
    Self { 
      clips: Vec::new(),
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
                      if ui.button("Copy").clicked() {
                        let mut clipboard: Clipboard = Clipboard::new().unwrap();
                        clipboard.set_text(text.clone()).unwrap();
                      };
                    ui.add(egui::Separator::default());
                  },
                  ClipBoardContent::ImageContent(_image) => { },
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
        _id: self.clips.len(),
        content: Some(clip),
        _texture: None,
      };
      self.clips.push(new_clip);
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
  let clipboard_content:Option<ClipBoardContent> = get_clipboard_content();
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
      ClipBoardContent::ImageContent(image) => {
        if let Some(last_content) = last_clip_content {
          match last_content {
            ClipBoardContent::ImageContent(last_image) => {
              if &last_image.bytes.as_ref() != &image.bytes.as_ref() {
                return Some(ClipBoardContent::ImageContent(image));
              }
            },
            _ => return None,
          }
        }
      },
    }
  }

  None
}

fn _load_image_from_memory(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
  let image: DynamicImage = image::load_from_memory(image_data)?;
  let size: [usize; 2] = [image.width() as _, image.height() as _];
  let image_buffer = image.to_rgba8();
  let pixels: FlatSamples<&[u8]> = image_buffer.as_flat_samples();
  Ok(ColorImage::from_rgba_unmultiplied(
      size,
      pixels.as_slice(),
  ))
}

fn _create_texture(id: usize, image: &ImageData, ui: &mut Ui) -> Option<egui::TextureHandle> {
  let img: Result<ColorImage, ImageError> = _load_image_from_memory(image.bytes.as_ref());

  Some(ui.ctx().load_texture(
    format!("img_{}", id),
    img.unwrap(),
    egui::TextureFilter::Linear
  ))
}

