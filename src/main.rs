use ray_tracing::{write_color, Color};

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");
    for j in 0..image_height {
        let remaining = image_height - j;
        eprintln!("Scanlines remaining {remaining}");
        for i in 0..image_width {
            let pixel_color = Color {
                e: [
                    i as f64 / (image_width - 1) as f64,
                    j as f64 / (image_height - 1) as f64,
                    0.0,
                ],
            };

            write_color(&pixel_color);
        }
    }
    eprintln!("Done.");
}
