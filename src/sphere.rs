use crate::dot;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Box<dyn Material>,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Box<dyn Material>) -> Self {
        Sphere {
            center: Ray::new_with_default_time(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius: if radius > 0.0 { radius } else { 0.0 },
            mat,
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Box<dyn Material>,
    ) -> Self {
        Sphere {
            center: Ray::new_with_default_time(center1, center2 - center1),
            radius: if radius > 0.0 { radius } else { 0.0 },
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            let mut root = (h - sqrtd) / a;
            if !ray_t.surrounds(root) {
                root = (h + sqrtd) / a;
                if !ray_t.surrounds(root) {
                    None
                } else {
                    let outward_normal = (r.at(root) - current_center) / self.radius;
                    let rec = HitRecord::new(root, r, outward_normal, &self.mat);
                    Some(rec)
                }
            } else {
                let outward_normal = (r.at(root) - current_center) / self.radius;
                let rec = HitRecord::new(root, r, outward_normal, &self.mat);
                Some(rec)
            }
        }
    }
}
