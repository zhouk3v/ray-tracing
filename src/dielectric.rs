use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRes};
use crate::ray::Ray;
use crate::vec3::{dot, reflect, refract, unit_vector};

pub struct Dielectric {
    refraction_index: f64, // Refractive index in vacuum or air, or the ratio of the material's refractive index over
                           // the refractive index of the enclosing media
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRes> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(*r_in.direction());
        let cos_theta = if dot(&(-unit_direction), &rec.normal) < 1.0 {
            dot(&(-unit_direction), &rec.normal)
        } else {
            1.0
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };

        Some(ScatterRes::new(
            Color::new(1.0, 1.0, 1.0),
            Ray::new(rec.p, direction),
        ))
    }
}
