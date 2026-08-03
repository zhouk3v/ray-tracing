use image::{ImageError, ImageReader, Rgb32FImage};

use crate::primitives::color::Color;
use crate::primitives::interval::Interval;
use crate::primitives::point3::Point3;
use crate::textures::texture::Texture;

pub struct ImageTexture {
    image: Option<Rgb32FImage>,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        match Self::read_image(filename) {
            Ok(image) => ImageTexture { image: Some(image) },
            Err(e) => {
                eprintln!("Error when reading {filename}: {e}");
                ImageTexture { image: None }
            }
        }
    }

    fn read_image(filename: &str) -> Result<Rgb32FImage, ImageError> {
        let image = ImageReader::open(filename)?.decode()?;
        Ok(image.to_rgb32f())
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match &self.image {
            Some(image) => {
                let u = Interval::new(0.0, 1.0).clamp(u);
                let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);
                let i = (u * image.width() as f64) as u32;
                let j = (v * image.height() as f64) as u32;
                let pixel = image.get_pixel(i, j);

                let color_scale = 1.0 / 255.0;
                Color::new(
                    color_scale * pixel[0] as f64,
                    color_scale * pixel[1] as f64,
                    color_scale * pixel[2] as f64,
                )
            }
            None => Color::new(0.0, 1.1, 1.1),
        }
    }
}
