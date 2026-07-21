use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct ScatterRes {
    attenuation: Color,
    scattered: Ray,
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
