use super::material::{Material, ScatterRes};
use crate::hittables::hittable::HitRecord;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::textures::solid_color::SolidColor;
use crate::textures::texture::Texture;

pub struct DiffuseLight<T: Texture> {
    tex: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(tex: T) -> Self {
        Self { tex }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn new_from_color(emit: Color) -> Self {
        Self {
            tex: SolidColor::new(emit),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRes> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}
