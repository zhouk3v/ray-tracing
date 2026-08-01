use crate::hittables::hittable::HitRecord;
use crate::primitives::color::Color;
use crate::primitives::ray::Ray;

pub struct ScatterRes {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl ScatterRes {
    pub fn new(attenuation: Color, scattered: Ray) -> Self {
        ScatterRes {
            attenuation,
            scattered,
        }
    }
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRes>;
}
