use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;

pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Aabb { x, y, z }
    }

    pub fn new_from_points(a: &Point3, b: &Point3) -> Self {
        Aabb {
            x: if a[0] <= b[0] {
                Interval::new(a[0], b[0])
            } else {
                Interval::new(b[0], a[0])
            },
            y: if a[1] <= b[1] {
                Interval::new(a[1], b[1])
            } else {
                Interval::new(b[1], a[1])
            },
            z: if a[2] <= b[2] {
                Interval::new(a[2], b[2])
            } else {
                Interval::new(b[2], a[2])
            },
        }
    }

    pub fn new_from_aabb(box0: &Aabb, box1: &Aabb) -> Self {
        Aabb {
            x: Interval::new_from_intervals(&box0.x, &box1.x),
            y: Interval::new_from_intervals(&box0.y, &box1.y),
            z: Interval::new_from_intervals(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Interval> {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        let mut res = Interval {
            min: ray_t.min,
            max: ray_t.max,
        };

        for axis in 0..3 {
            let axis_usize = axis as usize;
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis_usize];

            let t0 = (ax.min - ray_orig[axis_usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis_usize]) * adinv;

            if t0 < t1 {
                if t0 > res.min {
                    res.min = t0;
                }
                if t1 < res.max {
                    res.max = t1
                }
            } else {
                if t1 > res.min {
                    res.min = t1;
                }
                if t0 < res.max {
                    res.max = t0
                }
            }

            if res.max <= res.min {
                return None;
            }
        }
        Some(res)
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }
}
