use crate::camera::{Camera, CameraFocus, CameraPerformance, CameraPosition, ImageDimensions};
use crate::hittables::bvh_node::BVHNode;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::sphere::Sphere;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;
use crate::primitives::vec3::Vec3;
use crate::textures::checker_texture::CheckerTexture;

pub fn bouncing_spheres() {
    let mut objects = HittableList::new();

    let checker =
        CheckerTexture::new_with_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Lambertian::new_with_texture(checker);
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    let center2 = center + Vec3::new(0.0, rand::random_range(0.0..0.5), 0.0);
                    objects.add(Box::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_with_min_max(0.5, 1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    objects.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    objects.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    objects.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    objects.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut world = HittableList::new();
    world.add(Box::new(BVHNode::new(objects)));

    let image_dim = ImageDimensions::new(16.0 / 9.0, 1200.0);

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
