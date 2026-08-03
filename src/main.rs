use ray_tracing::scenes::bouncing_spheres::bouncing_spheres;
use ray_tracing::scenes::checkered_spheres::checkered_spheres;
use ray_tracing::scenes::earth::earth;

enum Scene {
    BOUNCING,
    CHECKERED,
    EARTH,
}

fn main() {
    let scene = Scene::EARTH;
    match scene {
        Scene::BOUNCING => bouncing_spheres(),
        Scene::CHECKERED => checkered_spheres(),
        Scene::EARTH => earth(),
    }
}
