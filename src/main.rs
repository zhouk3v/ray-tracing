use ray_tracing::{Camera, HittableList, Point3, Sphere};

fn main() {
    let mut world = HittableList::new();

    // world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    // world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;

    let cam = Camera::new(aspect_ratio, image_width, 100, 50);

    cam.render(&world);
}
