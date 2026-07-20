use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

/*
* Switchable stream version
*/

// use std::io::{self, Write};

// pub fn write_color(pixel_color: &Color) -> io::Result<()> {
//     let r = pixel_color.x();
//     let g = pixel_color.y();
//     let b = pixel_color.z();

//     let rbyte = (255.999 * r) as i64;
//     let gbyte = (255.999 * g) as i64;
//     let bbyte = (255.99 * b) as i64;

//     writeln!(out, "{rbyte} {gbyte} {bbyte}")?;
//     Ok(())
// }

/*
* println! version
*/

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(pixel_color: &Color) {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let intensity = Interval::new(0.000, 0.999);

    let rbyte = (256.0 * intensity.clamp(r)) as i64;
    let gbyte = (256.0 * intensity.clamp(g)) as i64;
    let bbyte = (256.0 * intensity.clamp(b)) as i64;

    println!("{rbyte} {gbyte} {bbyte}")
}
