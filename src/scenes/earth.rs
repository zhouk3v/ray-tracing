use crate::camera::{Camera, CameraFocus, CameraPerformance, CameraPosition, ImageDimensions};
use crate::hittables::hittable_list::HittableList;
use crate::hittables::sphere::Sphere;
use crate::materials::lambertian::Lambertian;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;
use crate::primitives::vec3::Vec3;
use crate::textures::image_texture::ImageTexture;

pub fn earth() {
    let earth_texture = Box::new(ImageTexture::new("assets/earthmap.jpg"));

    let earth_surface = Box::new(Lambertian::new_with_texture(earth_texture));

    let globe = Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut world = HittableList::new();
    world.add(globe);

    let image_dim = ImageDimensions::new(16.0 / 9.0, 400.0);

    let cam_performance = CameraPerformance::new(100, 50);

    let cam_position = CameraPosition::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
    );

    let cam_focus = CameraFocus::new(0.6, 10.0);

    let cam_background = Color::new(0.7, 0.8, 1.0);

    let cam = Camera::new(
        image_dim,
        cam_performance,
        cam_position,
        cam_focus,
        cam_background,
    );

    cam.render(&world);
}
