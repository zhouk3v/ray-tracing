use crate::dot;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(t: f64, r: &Ray, outward_normal: Vec3) -> Self {
        let p = r.at(t);
        let front_face = dot(r.direction(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}
