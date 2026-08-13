use rayon::prelude::*;

use crate::hittables::hittable::Hittable;
use crate::primitives::color::{write_color, Color};
use crate::primitives::interval::Interval;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::{cross, random_in_unit_disk, unit_vector, Vec3};

pub struct ImageDimensions {
    aspect_ratio: f64, // Ratio of image width over height
    image_width: f64,  // Rendered image width in pixel count
}

impl ImageDimensions {
    pub fn new(aspect_ratio: f64, image_width: f64) -> Self {
        Self {
            aspect_ratio,
            image_width,
        }
    }
}

pub struct CameraPerformance {
    samples_per_pixel: i32, // Count of random samples for each pixel
    max_depth: u32,         // Maximum number of ray bounces into scene
}

impl CameraPerformance {
    pub fn new(samples_per_pixel: i32, max_depth: u32) -> Self {
        Self {
            samples_per_pixel,
            max_depth,
        }
    }
}

pub struct CameraPosition {
    lookfrom: Point3, // Point camera is looking from
    lookat: Point3,   // Point camera is looking at
    vup: Vec3,        // Camera-relative "up" direction
    vfov: f64,        // Vertical view angle (field of view)
}

impl CameraPosition {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64) -> Self {
        CameraPosition {
            lookfrom,
            lookat,
            vup,
            vfov,
        }
    }
}

pub struct CameraFocus {
    defocus_angle: f64, // Variation angle of rays through each pixel
    focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus
}

impl CameraFocus {
    pub fn new(defocus_angle: f64, focus_dist: f64) -> Self {
        Self {
            defocus_angle,
            focus_dist,
        }
    }
}

pub struct Camera {
    image_width: f64,         // Rendered image width in pixel count
    samples_per_pixel: i32,   // Count of random samples for each pixel
    image_height: f64,        // Rendered image height
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel 0,0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    max_depth: u32,           // Maximum number of ray bounces into scene
    defocus_angle: f64,       // Variation angle of rays through each pixel
    defocus_disk_u: Vec3,     // Defocus disk horizontal radius
    defocus_disk_v: Vec3,     // Defocus disk vertical radius
    background: Color,        // Scene background color
}

impl Camera {
    pub fn new(
        image_dimensions: ImageDimensions,
        camera_performance: CameraPerformance,
        position: CameraPosition,
        focus: CameraFocus,
        background: Color,
    ) -> Self {
        let image_width = image_dimensions.image_width;
        let aspect_ratio = image_dimensions.aspect_ratio;
        let center = position.lookfrom;
        let vfov = position.vfov;
        let samples_per_pixel = camera_performance.samples_per_pixel;
        let max_depth = camera_performance.max_depth;
        let defocus_angle = focus.defocus_angle;
        let focus_dist = focus.focus_dist;

        let mut image_height = image_width / aspect_ratio;
        image_height = if image_height < 1.0 {
            1.0
        } else {
            image_height
        };

        // Determine viewport dimensions
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width / image_height);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        let w = unit_vector(position.lookfrom - position.lookat);
        let u = unit_vector(cross(&position.vup, &w));
        let v = cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector across viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background,
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
                let pixel_color: Color = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map(|_| {
                        let r = self.get_ray(i as f64, j as f64);
                        self.ray_color(&r, self.max_depth, world)
                    })
                    .sum();
                write_color(&(self.pixel_samples_scale * pixel_color));
            }
        }
        eprintln!("Done.");
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = self.sample_square();

        let pixel_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        let ray_time = rand::random::<f64>();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0,
        )
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn ray_color(&self, r: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth == 0 {
            Color::new(0.0, 0.0, 0.0)
        } else if let Some(rec) = world.hit(r, &Interval::new(0.001, f64::INFINITY)) {
            let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);
            if let Some(scatter) = rec.mat.scatter(r, &rec) {
                let color_from_scatter =
                    scatter.attenuation * self.ray_color(&scatter.scattered, depth - 1, world);
                color_from_emission + color_from_scatter
            } else {
                color_from_emission
            }
        } else {
            // If ray hits nothing, return the background color
            self.background
        }
    }
}
