use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BVHNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl BVHNode {
    pub fn new(hittable_list: HittableList) -> Self {
        Self::new_from_vec(hittable_list.objects)
    }

    fn new_from_vec(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        let mut bbox = Aabb::EMPTY;

        let objects_iter = objects.iter();

        for object in objects_iter {
            bbox = Aabb::new_from_aabb(&bbox, object.bounding_box())
        }

        let axis = bbox.longest_axis();

        objects.sort_by(|a, b| {
            let a_axis_interval = a.bounding_box().axis_interval(axis);
            let b_axis_interval = b.bounding_box().axis_interval(axis);
            a_axis_interval.min.total_cmp(&b_axis_interval.min)
        });

        match objects.len() {
            0 => BVHNode {
                left: None,
                right: None,
                bbox,
            },
            1 => {
                let obj = objects.pop().unwrap();
                BVHNode {
                    left: Some(obj),
                    right: None,
                    bbox,
                }
            }
            2 => {
                let right = objects.pop().unwrap();
                let left = objects.pop().unwrap();
                BVHNode {
                    left: Some(left),
                    right: Some(right),
                    bbox,
                }
            }
            len => {
                let right_half = objects.split_off(len / 2);
                let left_node = BVHNode::new_from_vec(objects);
                let right_node = BVHNode::new_from_vec(right_half);
                BVHNode {
                    left: Some(Box::new(left_node)),
                    right: Some(Box::new(right_node)),
                    bbox,
                }
            }
        }
    }

    fn hit_left(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        if let Some(left) = &self.left {
            left.hit(r, ray_t)
        } else {
            None
        }
    }

    fn hit_right(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        if let Some(right) = &self.right {
            right.hit(r, ray_t)
        } else {
            None
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        if self.bbox.hit(r, ray_t).is_some() {
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
