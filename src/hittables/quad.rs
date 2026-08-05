use super::hittable::{HitRecord, Hittable};
use crate::materials::material::Material;
use crate::primitives::aabb::Aabb;
use crate::primitives::interval::Interval;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::{cross, dot, unit_vector, Vec3};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Box<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Box<dyn Material>) -> Self {
        let bbox_diagonal1 = Aabb::new_from_points(&q, &(q + u + v));
        let bbox_diagonal2 = Aabb::new_from_points(&(q + u), &(q + v));
        let n = cross(&u, &v);
        let normal = unit_vector(n);
        Self {
            q,
            u,
            v,
            mat,
            bbox: Aabb::new_from_aabb(&bbox_diagonal1, &bbox_diagonal2),
            normal,
            d: dot(&normal, &q),
        }
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        let denom = dot(&self.normal, r.direction());

        if denom.abs() < 1.0e-8 {
            None
        } else {
            let t = (self.d - dot(&self.normal, r.origin())) / denom;
            if !ray_t.contains(t) {
                None
            } else {
                let rec = HitRecord::new(t, r, self.normal, self.mat.as_ref(), 0.0, 0.0);
                Some(rec)
            }
        }
    }
}
