use std::path::PathBuf;

use eframe::egui::{
    self, Color32, ColorImage, TextureFilter, TextureHandle, TextureOptions, TextureWrapMode,
};

use crate::palette::{Color, Palette};

pub struct MemoryImage {
    pub handle: TextureHandle,
    data: Vec<Color>,
    dimensions: [usize; 2],
}

impl MemoryImage {
    pub fn load_from_path(ctx: &egui::Context, path: &PathBuf) -> Option<MemoryImage> {
        let image_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let image = match image::open(path) {
            Ok(image) => image.into_rgb8(),
            Err(e) => {
                println!("Failed to load iamge: {e:?}");
                return None;
            }
        };

        let mut data = Vec::new();
        let dimensions = image.dimensions();
        let dimensions = [dimensions.0 as usize, dimensions.1 as usize];
        let mut image_data = ColorImage::new(dimensions, Color32::BLACK);

        for (index, pixel) in image.pixels().enumerate() {
            data.push(Color {
                x: pixel.0[0],
                y: pixel.0[1],
                z: pixel.0[2],
            });

            image_data.pixels[index] =
                Color32::from_rgba_unmultiplied(pixel.0[0], pixel.0[1], pixel.0[2], 255)
        }

        let handle = ctx.load_texture(
            image_name,
            image_data,
            TextureOptions {
                magnification: TextureFilter::Nearest,
                minification: TextureFilter::Nearest,
                wrap_mode: TextureWrapMode::ClampToEdge,
                mipmap_mode: None,
            },
        );

        Some(MemoryImage {
            handle,
            data,
            dimensions,
        })
    }

    pub fn load_preview(
        &self,
        ctx: &egui::Context,
        palette: &Palette,
        use_lab: bool,
    ) -> MemoryImage {
        let mut new_colors = Vec::new();
        let mut color_data = ColorImage::new(self.dimensions, Color32::BLACK);

        for (index, pixel) in self.data.iter().enumerate() {
            let palette_index = if use_lab {
                pixel.to_palette_index_lab(palette)
            } else {
                pixel.to_palette_index_rbg(palette)
            };
            let new_color = palette.colors[palette_index].clone();
            new_colors.push(new_color.clone());
            color_data.pixels[index] =
                Color32::from_rgba_unmultiplied(new_color.x, new_color.y, new_color.z, 255)
        }

        let handle = ctx.load_texture(
            "preview image",
            color_data,
            TextureOptions {
                magnification: TextureFilter::Nearest,
                minification: TextureFilter::Nearest,
                wrap_mode: TextureWrapMode::ClampToEdge,
                mipmap_mode: None,
            },
        );

        MemoryImage {
            handle: handle,
            data: new_colors,
            dimensions: self.dimensions,
        }
    }
}
