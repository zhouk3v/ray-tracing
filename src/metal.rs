use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRes};
use crate::ray::Ray;
use crate::vec3::{dot, random_unit_vector, reflect, unit_vector};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRes> {
        let reflected = reflect(r_in.direction(), &rec.normal);
        let reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        if dot(scattered.direction(), &rec.normal) > 0.0 {
            Some(ScatterRes::new(self.albedo, scattered))
        } else {
            None
        }
    }
}
