use crate::dot;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        // TODO: Initialize Box to material
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - *r.origin();
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
                    let outward_normal = (r.at(root) - self.center) / self.radius;
                    let rec = HitRecord::new(root, r, outward_normal, &self.mat);
                    Some(rec)
                }
            } else {
                let outward_normal = (r.at(root) - self.center) / self.radius;
                let rec = HitRecord::new(root, r, outward_normal, &self.mat);
                Some(rec)
            }
        }
    }
}
