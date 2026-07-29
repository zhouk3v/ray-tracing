use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: Aabb,
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(_) = self.bbox.hit(r, ray_t) {
            if let Some(hit_left) = self.left.hit(r, ray_t) {
                if let Some(hit_right) = self.right.hit(r, &Interval::new(ray_t.min, hit_left.t)) {
                    Some(hit_right)
                } else {
                    Some(hit_left)
                }
            } else {
                self.right.hit(r, &Interval::new(ray_t.min, ray_t.max))
            }
        } else {
            None
        }
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
