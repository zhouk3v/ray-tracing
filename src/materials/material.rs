use crate::hittables::hittable::HitRecord;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;
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

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRes>;

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
