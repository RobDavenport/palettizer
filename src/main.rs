use std::collections::HashSet;

use eframe::{
    egui::{self, Color32, ColorImage, TextureFilter, TextureOptions, TextureWrapMode},
    NativeOptions,
};
use lab::Lab;
use memory_image::MemoryImage;
use palette::{Color, Palette};

mod memory_image;
mod palette;

fn main() -> eframe::Result {
    eframe::run_native(
        "Palettizer",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<PalettizerApp>::default())),
    )
}

#[derive(Default)]
struct PalettizerApp {
    palette: Option<Palette>,

    input_images: Option<Vec<MemoryImage>>,

    preview_input_image_index: Option<usize>,
    preview_output_image: Option<MemoryImage>,

    use_lab: bool,
}

impl PalettizerApp {
    fn try_load_palette(ctx: &egui::Context) -> Option<Palette> {
        let path = match rfd::FileDialog::new()
            .set_title("Load a Palette")
            .set_directory("/")
            .add_filter(
                "image (.png, .jpeg, .gif, .bmp, .ico, .tiff, .tga)",
                &["png", "jpeg", "gif", "bmp", "ico", "tiff", "tga"],
            )
            .pick_file()
        {
            Some(path) => path,
            None => {
                println!("No image selected");
                return None;
            }
        };

        let image = match image::open(path) {
            Ok(image) => image.into_rgb8(),
            Err(e) => {
                println!("Failed to load iamge: {e:?}");
                return None;
            }
        };

        let mut unique_colors = HashSet::new();

        for pixel in image.pixels() {
            unique_colors.insert(Color {
                x: pixel.0[0],
                y: pixel.0[1],
                z: pixel.0[2],
            });
        }

        let mut image = ColorImage::new([unique_colors.len(), 1], Color32::BLACK);
        let mut labs = Vec::new();
        let colors = unique_colors
            .into_iter()
            .enumerate()
            .map(|(index, color)| {
                image.pixels[index] = Color32::from_rgb(color.x, color.y, color.z);
                labs.push(Lab::from_rgb(&[color.x, color.y, color.z]));
                color
            })
            .collect();

        let handle = ctx.load_texture(
            "default palette texture",
            image,
            TextureOptions {
                magnification: TextureFilter::Nearest,
                minification: TextureFilter::Nearest,
                wrap_mode: TextureWrapMode::ClampToEdge,
                mipmap_mode: None,
            },
        );

        Some(Palette {
            colors,
            handle,
            labs,
        })
    }

    fn load_input_images(ctx: &egui::Context) -> Option<Vec<MemoryImage>> {
        let paths = match rfd::FileDialog::new()
            .set_title("Load a Palette")
            .set_directory("/")
            .add_filter(
                "image (.png, .jpeg, .gif, .bmp, .ico, .tiff, .tga)",
                &["png", "jpeg", "gif", "bmp", "ico", "tiff", "tga"],
            )
            .pick_files()
        {
            Some(path) => path,
            None => {
                println!("No images selected");
                return None;
            }
        };

        let out: Vec<MemoryImage> = paths
            .iter()
            .flat_map(|path| MemoryImage::load_from_path(ctx, path))
            .collect();

        if out.is_empty() {
            return None;
        } else {
            Some(out)
        }
    }
}

impl eframe::App for PalettizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Palettizer");

            let mut should_recalculate = false;

            if ui.button("Load Palette").clicked() {
                if let Some(palette) = Self::try_load_palette(ctx) {
                    self.palette = Some(palette);
                    should_recalculate = true;
                }
            }

            if self.use_lab {
                if ui.button("Lab: Enabled").clicked() {
                    self.use_lab = false;
                    should_recalculate = true;
                }
            } else {
                if ui.button("Lab: Disabled").clicked() {
                    self.use_lab = true;
                    should_recalculate = true;
                }
            }

            if ui.button("Load Input Images").clicked() {
                if let Some(inputs) = Self::load_input_images(ctx) {
                    self.input_images = Some(inputs);
                    should_recalculate = true;
                }
            }

            if ui.button("Export Images").clicked() {
                print!("TODO: Export Images")
            }

            ui.separator();

            if let Some(palette) = &self.palette {
                ui.image(&palette.handle);
            }

            if let Some(images) = &self.input_images {
                for image in images {
                    ui.image(&image.handle);
                }
            }

            ui.separator();

            if should_recalculate {
                if let (Some(palette), Some(images)) = (&self.palette, &self.input_images) {
                    self.preview_output_image =
                        Some(images[0].load_preview(ctx, palette, self.use_lab))
                }
            }

            if let Some(preview) = &self.preview_output_image {
                ui.image(&preview.handle);
            }
        });
    }
}
