use crate::camera::{Camera, CameraFocus, CameraPerformance, CameraPosition, ImageDimensions};
use crate::hittables::hittable_list::HittableList;
use crate::hittables::quad::Quad;
use crate::hittables::sphere::Sphere;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;
use crate::primitives::vec3::Vec3;
use crate::textures::noise_texture::NoiseTexture;

pub fn simple_light() {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new_with_texture(NoiseTexture::new(4.0))),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Box::new(Lambertian::new_with_texture(NoiseTexture::new(4.0))),
    )));

    let difflight = Box::new(DiffuseLight::new_from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Box::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let difflight = Box::new(DiffuseLight::new_from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));

    let image_dim = ImageDimensions::new(16.0 / 9.0, 400.0);

    let cam_performance = CameraPerformance::new(100, 50);

    let cam_position = CameraPosition::new(
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
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
