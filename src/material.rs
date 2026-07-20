use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct ScatterRes {
    attenuation: Color,
    scattered: Ray,
}

pub trait Material {
    fn scatter(r_in: &Ray, rec: &HitRecord) -> ScatterRes;
}
