use super::point3::Point3;
use super::vec3::Vec3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }

    pub fn new_with_default_time(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            tm: 0.0,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            orig: Point3::default(),
            dir: Point3::default(),
            tm: 0.0,
        }
    }
}
