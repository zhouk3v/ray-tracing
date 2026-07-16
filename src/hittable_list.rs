use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::Interval;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far_interval = Interval::new(ray_t.min, ray_t.max);

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, &closest_so_far_interval) {
                closest_so_far_interval.max = rec.t;
                temp_rec = Some(rec)
            }
        }
        temp_rec
    }
}
