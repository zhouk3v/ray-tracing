use std::f64::consts::PI;

use super::hittable::{HitRecord, Hittable};
use crate::materials::material::Material;
use crate::primitives::aabb::Aabb;
use crate::primitives::interval::Interval;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::{dot, Vec3};

struct SphereUV {
    u: f64,
    v: f64,
}
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Box<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Box<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center: Ray::new_with_default_time(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius: if radius > 0.0 { radius } else { 0.0 },
            mat,
            bbox: Aabb::new_from_points(&(static_center - rvec), &(static_center + rvec)),
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Box<dyn Material>,
    ) -> Self {
        let center = Ray::new_with_default_time(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_from_points(&(center.at(0.0) - rvec), &(center.at(0.0) + rvec));
        let box2 = Aabb::new_from_points(&(center.at(1.0) - rvec), &(center.at(1.0) + rvec));
        Sphere {
            center,
            radius: if radius > 0.0 { radius } else { 0.0 },
            mat,
            bbox: Aabb::new_from_aabb(&box1, &box2),
        }
    }

    fn get_sphere_uv(&self, p: &Point3) -> SphereUV {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        SphereUV {
            u: phi / (2.0 * PI),
            v: theta / PI,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
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
                    let uv = self.get_sphere_uv(&outward_normal);
                    let rec =
                        HitRecord::new(root, r, outward_normal, self.mat.as_ref(), uv.u, uv.v);
                    Some(rec)
                }
            } else {
                let outward_normal = (r.at(root) - current_center) / self.radius;
                let uv = self.get_sphere_uv(&outward_normal);
                let rec = HitRecord::new(root, r, outward_normal, self.mat.as_ref(), uv.u, uv.v);
                Some(rec)
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
