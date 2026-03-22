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

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as i64;
    let gbyte = (255.999 * g) as i64;
    let bbyte = (255.99 * b) as i64;

    println!("{rbyte} {gbyte} {bbyte}")
}
