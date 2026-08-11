use super::material::{Material, ScatterRes};
use crate::hittables::hittable::HitRecord;
use crate::primitives::color::Color;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::random_unit_vector;
use crate::textures::solid_color::SolidColor;
use crate::textures::texture::Texture;

pub struct Lambertian<T: Texture> {
    tex: T,
}

impl Lambertian<SolidColor> {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            tex: SolidColor::new(albedo),
        }
    }
}

impl<T: Texture> Lambertian<T> {
    pub fn new_with_texture(tex: T) -> Self {
        Lambertian { tex }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRes> {
        let scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            Some(ScatterRes::new(
                self.tex.value(rec.u, rec.v, &rec.p),
                Ray::new(rec.p, rec.normal, r_in.time()),
            ))
        } else {
            Some(ScatterRes::new(
                self.tex.value(rec.u, rec.v, &rec.p),
                Ray::new(rec.p, scatter_direction, r_in.time()),
            ))
        }
    }
}
