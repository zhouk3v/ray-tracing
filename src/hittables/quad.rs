use super::hittable::{HitRecord, Hittable};
use crate::materials::material::Material;
use crate::primitives::aabb::Aabb;
use crate::primitives::interval::Interval;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::Vec3;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Box<dyn Material>,
    bbox: Aabb,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Box<dyn Material>) -> Self {
        let bbox_diagonal1 = Aabb::new_from_points(&q, &(q + u + v));
        let bbox_diagonal2 = Aabb::new_from_points(&(q + u), &(q + v));
        Self {
            q,
            u,
            v,
            mat,
            bbox: Aabb::new_from_aabb(&bbox_diagonal1, &bbox_diagonal2),
        }
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        None
    }
}
