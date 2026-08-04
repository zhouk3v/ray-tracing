use ray_tracing::scenes::bouncing_spheres::bouncing_spheres;
use ray_tracing::scenes::checkered_spheres::checkered_spheres;
use ray_tracing::scenes::earth::earth;
use ray_tracing::scenes::perlin_spheres::perlin_spheres;

enum Scene {
    BOUNCING,
    CHECKERED,
    EARTH,
    PERLIN,
}

fn main() {
    let scene = Scene::PERLIN;
    match scene {
        Scene::BOUNCING => bouncing_spheres(),
        Scene::CHECKERED => checkered_spheres(),
        Scene::EARTH => earth(),
        Scene::PERLIN => perlin_spheres(),
    }
}
