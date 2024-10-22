use eframe::egui::TextureHandle;
use lab::Lab;

#[derive(Clone)]
pub struct Palette {
    pub colors: Vec<Color>,
    pub labs: Vec<Lab>,
    pub handle: TextureHandle,
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct Color {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Color {
    // Convert color components to a 3D vector of floats
    fn to_vec3f(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }

    // Calculate the squared Euclidean distance between two Vec3f
    fn squared_distance(a: [f32; 3], b: [f32; 3]) -> f32 {
        let dx = a[0] - b[0];
        let dy = a[1] - b[1];
        let dz = a[2] - b[2];
        dx * dx + dy * dy + dz * dz
    }

    // Find the index of the closest matching color in the palette in RGB
    pub fn to_palette_index_rbg(&self, palette: &Palette) -> usize {
        let self_vec = self.to_vec3f();
        let mut closest_index = 0;
        let mut smallest_distance = f32::MAX;

        for (i, color) in palette.colors.iter().enumerate() {
            let palette_vec = color.to_vec3f();

            let distance = Self::squared_distance(self_vec, palette_vec);

            if distance < smallest_distance {
                smallest_distance = distance;
                closest_index = i;
            }
        }

        closest_index
    }

    // Find the index of the closest matching color in the palette in RGB
    pub fn to_palette_index_lab(&self, palette: &Palette) -> usize {
        let self_vec = Lab::from_rgb(&[self.x, self.y, self.z]);
        let self_vec = [self_vec.l, self_vec.a, self_vec.b];
        let mut closest_index = 0;
        let mut smallest_distance = f32::MAX;

        for (i, color) in palette.labs.iter().enumerate() {
            let palette_vec = [color.l as f32, color.a as f32, color.b as f32];
            let distance = Self::squared_distance(self_vec, palette_vec);

            if distance < smallest_distance {
                smallest_distance = distance;
                closest_index = i;
            }
        }

        closest_index
    }
}
