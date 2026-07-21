use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRes};
use crate::ray::Ray;
use crate::vec3::random_unit_vector;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRes> {
        let scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            Some(ScatterRes::new(self.albedo, Ray::new(rec.p, rec.normal)))
        } else {
            Some(ScatterRes::new(
                self.albedo,
                Ray::new(rec.p, scatter_direction),
            ))
        }
    }
}
