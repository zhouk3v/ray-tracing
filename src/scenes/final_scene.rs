use crate::camera::{Camera, CameraFocus, CameraPerformance, CameraPosition, ImageDimensions};
use crate::hittables::box_object::{BoxInstance, BoxInstanceSideMaterials};
use crate::hittables::bvh_node::BVHNode;
use crate::hittables::constant_medium::ConstantMedium;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::quad::Quad;
use crate::hittables::rotate_y::RotateY;
use crate::hittables::sphere::Sphere;
use crate::hittables::translate::Translate;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::primitives::color::Color;
use crate::primitives::point3::Point3;
use crate::primitives::vec3::Vec3;
use crate::textures::image_texture::ImageTexture;
use crate::textures::noise_texture::NoiseTexture;

pub fn final_scene(image_width: f64, samples_per_pixel: i32, max_depth: u32) {
    // Ground (Boxes of random height)
    let mut boxes1 = HittableList::new();

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::random_range(1.0..101.0);
            let z1 = z0 + w;

            let ground = BoxInstanceSideMaterials::new(
                Lambertian::new(Color::new(0.48, 0.83, 0.53)),
                Lambertian::new(Color::new(0.48, 0.83, 0.53)),
                Lambertian::new(Color::new(0.48, 0.83, 0.53)),
                Lambertian::new(Color::new(0.48, 0.83, 0.53)),
                Lambertian::new(Color::new(0.48, 0.83, 0.53)),
                Lambertian::new(Color::new(0.48, 0.83, 0.53)),
            );

            boxes1.add(Box::new(BoxInstance::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground,
            )))
        }
    }

    let mut world = HittableList::new();

    world.add(Box::new(BVHNode::new(boxes1)));

    // Light
    let light = DiffuseLight::new_from_color(Color::new(7.0, 7.0, 7.0));
    world.add(Box::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    // Moving Sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Box::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    // Glass Sphere
    world.add(Box::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Box::new(Dielectric::new(1.5)),
    )));

    // Metal Sphere
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Box::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // Blue subsurface reflection sphere
    let boundary = Box::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Box::new(Dielectric::new(1.5)),
    ));
    world.add(boundary);
    world.add(Box::new(ConstantMedium::new(
        Sphere::new(
            Point3::new(360.0, 150.0, 145.0),
            70.0,
            Box::new(Dielectric::new(1.5)),
        ),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    // Fog
    let boundary = Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Box::new(Dielectric::new(1.5)),
    );
    world.add(Box::new(ConstantMedium::new(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    // Earth
    let earth_texture = ImageTexture::new("assets/earthmap.jpg");
    let earth_surface = Box::new(Lambertian::new_with_texture(earth_texture));
    world.add(Box::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_surface,
    )));

    // Perlin Noise Sphere
    let pertext = Box::new(Lambertian::new_with_texture(NoiseTexture::new(0.2)));
    world.add(Box::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        pertext,
    )));

    // Cluster of spheres
    let mut boxes2 = HittableList::new();
    let ns = 1000;
    for _ in 0..ns {
        let white = Box::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
        boxes2.add(Box::new(Sphere::new(
            Point3::random_with_min_max(0.0, 165.0),
            10.0,
            white,
        )))
    }
    world.add(Box::new(Translate::new(
        RotateY::new(BVHNode::new(boxes2), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let image_dim = ImageDimensions::new(1.0, image_width);

    let cam_performance = CameraPerformance::new(samples_per_pixel, max_depth);

    let cam_position = CameraPosition::new(
        Point3::new(478.0, 278.0, -600.0),
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
