use ray_tracing::scenes::bouncing_spheres::bouncing_spheres;
use ray_tracing::scenes::checkered_spheres::checkered_spheres;

enum Scene {
    BOUNCING,
    CHECKERED,
}

fn main() {
    let scene = Scene::CHECKERED;
    match scene {
        Scene::BOUNCING => bouncing_spheres(),
        Scene::CHECKERED => checkered_spheres(),
    }
}
