use super::solid_color::SolidColor;
use super::texture::Texture;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;

pub struct CheckerTexture<T: Texture, U: Texture> {
    inv_scale: f64,
    even: T,
    odd: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(scale: f64, even: T, odd: U) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn new_with_color(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: SolidColor::new(c1),
            odd: SolidColor::new(c2),
        }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
