use crate::camera::{Camera, CameraFocus, CameraPerformance, CameraPosition, ImageDimensions};
use crate::hittables::hittable_list::HittableList;
use crate::hittables::quad::Quad;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;
use crate::primitives::vec3::Vec3;

pub fn cornell_box() {
    let mut world = HittableList::new();

    let green = Box::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    let red = Box::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    let light = Box::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));
    world.add(Box::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    let white = Box::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white,
    )));
    let white = Box::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white,
    )));
    let white = Box::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white,
    )));

    let image_dim = ImageDimensions::new(1.0, 600.0);

    let cam_performance = CameraPerformance::new(200, 50);

    let cam_position = CameraPosition::new(
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
    );

    let cam_focus = CameraFocus::new(0.0, 10.0);

    let cam_background = Color::new(0.0, 0.0, 0.0);

    let cam = Camera::new(
        image_dim,
        cam_performance,
        cam_position,
        cam_focus,
        cam_background,
    );

    cam.render(&world);
}
