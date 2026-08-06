use crate::camera::{Camera, CameraFocus, CameraPerformace, CameraPosition, ImageDimensions};
use crate::hittables::bvh_node::BVHNode;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::sphere::Sphere;
use crate::materials::lambertian::Lambertian;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;
use crate::primitives::vec3::Vec3;
use crate::textures::checker_texture::CheckerTexture;

pub fn checkered_spheres() {
    let mut objects = HittableList::new();

    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Box::new(Lambertian::new_with_texture(Box::new(
            CheckerTexture::new_with_color(
                0.32,
                Color::new(0.2, 0.3, 0.1),
                Color::new(0.9, 0.9, 0.9),
            ),
        ))),
    )));

    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Box::new(Lambertian::new_with_texture(Box::new(
            CheckerTexture::new_with_color(
                0.32,
                Color::new(0.2, 0.3, 0.1),
                Color::new(0.9, 0.9, 0.9),
            ),
        ))),
    )));

    let mut world = HittableList::new();
    world.add(Box::new(BVHNode::new(objects)));

    let image_dim = ImageDimensions::new(16.0 / 9.0, 1200.0);

    let cam_performance = CameraPerformace::new(100, 50, 20.0);

    let cam_position = CameraPosition::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let cam_focus = CameraFocus::new(0.0, 10.0);

    let cam = Camera::new(image_dim, cam_performance, cam_position, cam_focus);

    cam.render(&world);
}
