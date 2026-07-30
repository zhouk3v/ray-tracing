use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BVHNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl BVHNode {
    fn hit_left(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(left) = &self.left {
            left.hit(r, ray_t)
        } else {
            None
        }
    }

    fn hit_right(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(right) = &self.right {
            right.hit(r, ray_t)
        } else {
            None
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(_) = self.bbox.hit(r, ray_t) {
            if let Some(hit_left) = self.hit_left(r, ray_t) {
                if let Some(hit_right) = self.hit_right(r, &Interval::new(ray_t.min, hit_left.t)) {
                    Some(hit_right)
                } else {
                    Some(hit_left)
                }
            } else {
                self.hit_right(r, ray_t)
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
