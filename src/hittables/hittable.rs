use crate::materials::material::Material;
use crate::primitives::aabb::Aabb;
use crate::primitives::interval::Interval;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::{dot, Vec3};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>>;

    fn bounding_box(&self) -> &Aabb;
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        r: &Ray,
        outward_normal: Vec3,
        mat: &'a dyn Material,
        u: f64,
        v: f64,
    ) -> Self {
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
            u,
            v,
            front_face,
        }
    }
}
