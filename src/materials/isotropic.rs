use super::material::{Material, ScatterRes};

use crate::hittables::hittable::HitRecord;
use crate::primitives::color::Color;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::random_unit_vector;
use crate::textures::solid_color::SolidColor;
use crate::textures::texture::Texture;

pub struct Isotropic {
    tex: SolidColor,
}

impl Isotropic {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: SolidColor::new(albedo),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRes> {
        Some(ScatterRes::new(
            self.tex.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p, random_unit_vector(), r_in.time()),
        ))
    }
}
