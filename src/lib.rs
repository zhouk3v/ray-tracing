pub mod vec3;
pub mod point3;
pub mod color;

pub use vec3::{Vec3, dot, cross, unit_vector};
pub use point3::Point3;
pub use color::{Color, write_color};
