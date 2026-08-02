use ray_tracing::scenes::bouncing_spheres::bouncing_spheres;
use ray_tracing::scenes::checkered_spheres::checkered_spheres;

fn main() {
    let scene = 0;
    match scene {
        0 => bouncing_spheres(),
        1 => checkered_spheres(),
        _ => bouncing_spheres(),
    }
}
