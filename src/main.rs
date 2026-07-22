use ray_tracing::{
    Camera, Color, Dielectric, HittableList, Lambertian, Metal, Point3, Sphere, Vec3,
};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.00 / 1.50);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Box::new(material_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        Box::new(material_bubble),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right),
    )));

    // let r = (std::f64::consts::PI / 4.0).cos();

    // let material_left = Lambertian::new(Color::new(0.0, 0.0, 1.0));
    // let material_right = Lambertian::new(Color::new(1.0, 0.0, 0.0));

    // world.add(Box::new(Sphere::new(
    //     Point3::new(-r, 0.0, -1.0),
    //     r,
    //     Box::new(material_left),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(r, 0.0, -1.0),
    //     r,
    //     Box::new(material_right),
    // )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        100,
        50,
        20.0,
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render(&world);
}
