use super::material::{Material, ScatterRes};
use crate::hittables::hittable::HitRecord;
use crate::primitives::color::Color;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::random_unit_vector;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRes> {
        let scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            Some(ScatterRes::new(
                self.albedo,
                Ray::new(rec.p, rec.normal, r_in.time()),
            ))
        } else {
            Some(ScatterRes::new(
                self.albedo,
                Ray::new(rec.p, scatter_direction, r_in.time()),
            ))
        }
    }
}
