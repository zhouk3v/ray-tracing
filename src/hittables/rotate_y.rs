use super::hittable::{HitRecord, Hittable};
use crate::primitives::aabb::Aabb;
use crate::primitives::interval::Interval;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::Vec3;

pub struct RotateY<T: Hittable> {
    object: T,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl<T: Hittable> RotateY<T> {
    pub fn new(object: T, angle: f64) -> Self {
        let radians = angle.to_radians();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box();

        let mut min = [f64::INFINITY, f64::INFINITY, f64::INFINITY];

        let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_double = i as f64;
                    let j_double = j as f64;
                    let k_double = k as f64;

                    let x = i_double * bbox.x.max + (1.0 - i_double) * bbox.x.min;
                    let y = j_double * bbox.y.max + (1.0 - j_double) * bbox.y.min;
                    let z = k_double * bbox.z.max + (1.0 - k_double) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = [new_x, y, new_z];

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_from_points(
                &Point3::new(min[0], min[1], min[2]),
                &Point3::new(max[0], max[1], max[2]),
            ),
        }
    }
}

impl<T: Hittable> Hittable for RotateY<T> {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        // Transform the ray from world space to object space
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new(origin, direction, r.time());

        // Determine whether an intersection exists in object space (and if so, where)
        if let Some(mut rec) = self.object.hit(&rotated_r, ray_t) {
            // Transform the intersection from object space back to world space
            rec.p = Point3::new(
                (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
                rec.p.y(),
                (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
            );

            rec.normal = Vec3::new(
                (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
                rec.normal.y(),
                (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
            );

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
