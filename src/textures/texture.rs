use crate::primitives::color::Color;
use crate::primitives::point3::Point3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
