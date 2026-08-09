use super::hittable::{HitRecord, Hittable};
use crate::primitives::aabb::Aabb;
use crate::primitives::interval::Interval;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::Vec3;

pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = *object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        let offset_r = Ray::new(*r.origin() - self.offset, *r.direction(), r.time());

        if let Some(mut hit) = self.object.hit(&offset_r, ray_t) {
            hit.p += self.offset;
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
