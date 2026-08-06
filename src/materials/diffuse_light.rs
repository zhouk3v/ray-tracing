use super::material::{Material, ScatterRes};
use crate::hittables::hittable::HitRecord;
use crate::primitives::color::Color;
use crate::primitives::ray::Ray;
use crate::textures::solid_color::SolidColor;
use crate::textures::texture::Texture;

pub struct DiffuseLight {
    tex: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Box<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn new_from_color(emit: Color) -> Self {
        Self {
            tex: Box::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRes> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &crate::primitives::point3::Point3) -> Color {
        self.tex.value(u, v, p)
    }
}
