use crate::dot;
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, r: &Ray, outward_normal: Vec3, mat: &'a Box<dyn Material>) -> Self {
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
            mat,
            t,
            front_face,
        }
    }
}
