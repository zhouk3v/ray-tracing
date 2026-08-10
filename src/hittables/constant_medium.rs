use super::hittable::{HitRecord, Hittable};

use crate::materials::isotropic::Isotropic;
use crate::primitives::aabb::Aabb;
use crate::primitives::color::Color;
use crate::primitives::interval::Interval;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::Vec3;

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Box<Isotropic>,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Box::new(Isotropic::new(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        if let Some(mut rec1) = self.boundary.hit(r, &Interval::UNIVERSE) {
            if let Some(mut rec2) = self
                .boundary
                .hit(r, &Interval::new(rec1.t + 0.0001, f64::INFINITY))
            {
                if rec1.t < ray_t.min {
                    rec1.t = ray_t.min;
                }

                if rec2.t > ray_t.max {
                    rec2.t = ray_t.max;
                }

                if rec1.t >= rec2.t {
                    None
                } else {
                    if rec1.t < 0.0 {
                        rec1.t = 0.0;
                    }

                    let ray_length = r.direction().length();
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                    let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();

                    if hit_distance > distance_inside_boundary {
                        None
                    } else {
                        Some(HitRecord::new(
                            rec1.t + hit_distance / ray_length,
                            r,
                            Vec3::new(1.0, 0.0, 0.0),     // arbitrary
                            self.phase_function.as_ref(), // also arbitrary
                            0.0,                          // also arbitrary
                            0.0,                          // also arbitrary
                        ))
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
