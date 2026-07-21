use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRes};
use crate::ray::Ray;
use crate::vec3::reflect;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRes> {
        let reflected = reflect(r_in.direction(), &rec.normal);
        Some(ScatterRes::new(self.albedo, Ray::new(rec.p, reflected)))
    }
}
