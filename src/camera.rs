use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

pub struct Camera {
    image_width: f64,
    image_height: f64,   // Rendered image height
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0,0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: f64) -> Self {
        let mut image_height = image_width / aspect_ratio;
        image_height = if image_height < 1.0 {
            1.0
        } else {
            image_height
        };

        let center = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width / image_height);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        let image_width_int = self.image_width as i32;
        let image_height_int = self.image_height as i32;
        println!("P3");
        let image_width = self.image_width;
        let image_height = self.image_height;
        println!("{image_width} {image_height}");
        println!("255");
        for j in 0..image_height_int {
            let remaining = image_height_int - j;
            eprintln!("Scanlines remaining {remaining}");
            for i in 0..image_width_int {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);
                write_color(&pixel_color);
            }
        }
        eprintln!("Done.");
    }

    fn ray_color(&self, r: &Ray, world: &impl Hittable) -> Color {
        if let Some(rec) = world.hit(r, &Interval::new(0.0, f64::INFINITY)) {
            0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = unit_vector(*r.direction());
            let a = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}
