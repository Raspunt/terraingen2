use std::vec;

use fastnoise_lite::*;
use image::{ImageBuffer, Rgb};

pub struct HeightmapGenerator {
    heightmap: Vec<Vec<f32>>,
}

impl HeightmapGenerator {
    pub fn new() -> Self {
        HeightmapGenerator {
            heightmap: Vec::new(),
        }
    }

    pub fn show_heightmap(&self) {
        for row in &self.heightmap {
            for &height in row {
                print!("{:.2} ", height);
            }
            println!();
        }
    }

    pub fn get_heightmap(&self) -> &Vec<Vec<f32>> {
        &self.heightmap
    }

    fn lerp(start: f32, end: f32, percent: f32) -> f32 {
        start + percent * (end - start)
    }

    pub fn generate_heightmap(
        &mut self,
        mut height: i32,
        mut width: i32,
        prev_hmap: Option<&Vec<Vec<f32>>>,
    ) -> Vec<Vec<f32>> {
        // Doubling the width and height as per your original code
        width *= 2;
        height *= 2;

        let mut fnoise = FastNoiseLite::new();
        fnoise.set_frequency(Some(0.1));
        fnoise.set_noise_type(Some(NoiseType::Perlin));

        let mut heightmap = vec![vec![0.0; width as usize]; height as usize];

        for z in 0..height {
            for x in 0..width {
                let noise_value = fnoise.get_noise_2d(x as f32, z as f32);
                heightmap[z as usize][x as usize] = noise_value;
            }
        }

        self.heightmap = heightmap.clone();
        heightmap
    }

    pub fn create_heightmap_image(&self) {
        let height = self.heightmap.len();
        let width = self.heightmap[0].len();
        let mut imgbuf = ImageBuffer::new(width as u32, height as u32);

        for (y, row) in self.heightmap.iter().enumerate() {
            for (x, &height_value) in row.iter().enumerate() {
                let pixel_value = (height_value * 255.0) as u8;
                imgbuf.put_pixel(
                    x as u32,
                    y as u32,
                    Rgb([pixel_value, pixel_value, pixel_value]),
                );
            }
        }

        imgbuf.save("data/heightmap.png").unwrap();
    }
}
