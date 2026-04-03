use crate::dot;
use crate::hittable::{HitRecord, Hittable};
use crate::point3::Point3;
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            false
        } else {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            let mut root = (h - sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                root = (h + sqrtd) / a;
                if root <= ray_tmin || ray_tmax <= root {
                    false
                } else {
                    rec.t = root;
                    rec.p = r.at(rec.t);
                    rec.normal = (rec.p - self.center) / self.radius;
                    true
                }
            } else {
                rec.t = root;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                true
            }
        }
    }
}
