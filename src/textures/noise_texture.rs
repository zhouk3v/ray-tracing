use super::perlin::Perlin;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;
use crate::textures::texture::Texture;

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::default(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let p_new = Point3::new(p.x(), p.y(), p.z());
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(p_new * self.scale)))
    }
}
