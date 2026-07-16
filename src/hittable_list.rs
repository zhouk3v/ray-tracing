use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

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
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec)
            }
        }
        temp_rec
    }
}
